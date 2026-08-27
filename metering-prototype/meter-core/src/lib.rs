//! Phase B of the metering prototype: the D3b terminal-work meter, modelled in memory.
//!
//! This is the meter ARITHMETIC exactly as DESIGN.md v12 states it, with no store, no consensus,
//! and no networking. It is pure deterministic bookkeeping over a block loop. Its job is to make
//! the ten certified invariants (docs/METERING_PROTOTYPE_SPEC.md, section 6) executable and to
//! demonstrate them holding under the certified lifecycle transitions and FAILING under a
//! deliberately broken variant. The storage-dimension units come from Phase A calibration
//! (docs/METERING_RESULTS.md); the numbers here are illustrative capacities chosen to exercise
//! the arithmetic, not claims about the platform.
//!
//! Terminology map to the design:
//!   - "terminal work" is the worst-case ending-cost an object imposes, a per-dimension vector.
//!   - "drain lane" is the deadline-free service rate R; owner-paid ordinary-lane discharge is not
//!     drain work (v12 lane attribution).
//!   - "known_due" is dated (deadline-bearing) work reserved in advance.
//!   - C_total is the block's cleanup-and-terminal capacity, partitioned into known_due, the R
//!     share, and an overdue reserve.

use std::collections::{BTreeMap, VecDeque};

/// The per-dimension terminal-work vector. Three dimensions are modelled: permanent storage
/// (Phase A's density-independent added_bytes), propagation (parent-hash rewrites), and hashing.
/// Compute is a fourth dimension the design names but Phase B does not price (that is phase E), so
/// it is omitted here rather than modelled with a fabricated unit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub perm: u64,
    pub prop: u64,
    pub hash: u64,
}

impl Work {
    pub const ZERO: Work = Work { perm: 0, prop: 0, hash: 0 };

    pub fn new(perm: u64, prop: u64, hash: u64) -> Self {
        Work { perm, prop, hash }
    }

    /// Saturating componentwise addition, used for REPORTING accumulations (totals shown or stored)
    /// where a cap at u64::MAX is acceptable. It is NOT used for capacity DECISIONS: saturation still
    /// compares as `<=` a u64::MAX capacity, so at the ceiling an over-capacity sum could pass a `ge`
    /// check. Every admission, drain, partition, and burst decision therefore uses `checked_add` /
    /// `checked_mul_scalar` below and treats an overflow as refusal or an invariant violation.
    pub fn add(self, o: Work) -> Work {
        Work::new(
            self.perm.saturating_add(o.perm),
            self.prop.saturating_add(o.prop),
            self.hash.saturating_add(o.hash),
        )
    }

    /// Checked componentwise addition: `None` if any dimension would overflow u64. A capacity
    /// decision reads an overflow as "the true total exceeds the u64 range, hence exceeds any
    /// representable capacity", so it refuses rather than saturating into a passing comparison.
    pub fn checked_add(self, o: Work) -> Option<Work> {
        Some(Work::new(
            self.perm.checked_add(o.perm)?,
            self.prop.checked_add(o.prop)?,
            self.hash.checked_add(o.hash)?,
        ))
    }

    /// Checked componentwise scalar multiply: `None` if any dimension would overflow u64.
    pub fn checked_mul_scalar(self, n: u64) -> Option<Work> {
        Some(Work::new(
            self.perm.checked_mul(n)?,
            self.prop.checked_mul(n)?,
            self.hash.checked_mul(n)?,
        ))
    }

    /// Componentwise `self >= o`, the ordering the accounting invariant uses.
    pub fn ge(self, o: Work) -> bool {
        self.perm >= o.perm && self.prop >= o.prop && self.hash >= o.hash
    }

    /// The positive part of `self - other`, per dimension. This is the marginal charge on a
    /// growth or reclassification: only components that INCREASED are charged, and a component
    /// that decreased contributes zero rather than a credit.
    pub fn positive_delta(self, other: Work) -> Work {
        Work::new(
            self.perm.saturating_sub(other.perm),
            self.prop.saturating_sub(other.prop),
            self.hash.saturating_sub(other.hash),
        )
    }

    pub fn is_zero(self) -> bool {
        self == Work::ZERO
    }
}

/// The ownership class of a position, which determines its terminal-work vector and lane
/// attribution (v12: single-owner balance discharge is owner-paid and not drain work; autonomous
/// distribution and physical cleanup ARE drain work).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    SingleOwner,
    Autonomous,
    IrrevocableRequest,
}

/// The lifecycle state of an object under the meter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Live,
    /// Discharge authority has ended; the object's physical cleanup is queued and its accounting
    /// has moved to the queue item. It is not yet reclaimed.
    Discharged,
    /// Terminal: physical reclamation completed, funding released.
    Terminal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub id: u64,
    pub class: Class,
    /// The drain-lane worst-case terminal work (the portion that flows through R). Owner-lane
    /// work is tracked separately and is not metered against R.
    pub drain_work: Work,
    /// accounted(o): must be >= drain_work at every block boundary (invariant 1).
    pub accounted: Work,
    /// Prepaid funding held for this object's terminal work, released only at reclamation.
    pub funding: u64,
    pub state: State,
    /// Some(height) => dated known_due work; None => deadline-free rate-R drain.
    pub deadline: Option<u64>,
    /// Set true once physical reclamation completes, at which point funding may release.
    pub reclaimed: bool,
}

/// Deliberately broken variants for the mutation-check discipline. Each fault causes exactly one
/// invariant to be violated, so a test can watch that invariant FAIL under the fault and PASS
/// without it. A test does not exist until it has been watched failing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    None,
    /// inv1: admit an object under-accounted (accounted < drain_work).
    UnderAccount,
    /// inv2: admit a positive drain-lane delta beyond R in one block.
    OverAdmitFlow,
    /// inv3: partition the capacity so known_due + R + overdue > C_total.
    OverPartition,
    /// inv4: on transfer, credit the new position without discharging the old (balance in two
    /// places).
    TransferDoubleCredit,
    /// inv5: release funding at discharge instead of at reclamation.
    EarlyRelease,
    /// inv6: mutate state on a transition that was supposed to be refused.
    MutateOnRefuse,
    /// inv7: the drain skips dated work that is already due.
    MissDeadline,
    /// inv8: admit an irrevocability burst beyond the known_due share.
    OverIrrevocable,
    /// inv9: on an upward reclassification, raise drain_work to the new class but skip charging the
    /// positive delta, so the class change moves work into the lane uncharged (accounted < worst
    /// case). This is the "zero-growth" misread round 11 worried about.
    UnderChargeReclass,
    /// inv10: mass retirement leaves an object reachable in two terminal dispositions.
    DoubleTerminal,
}

/// A record of a balance move, used to check transfer conservation (invariant 4).
#[derive(Clone, Copy, Debug, PartialEq)]
struct BalanceEvent {
    debited: bool,
    credited: bool,
}

/// A complete, comparable snapshot of every mutable field of the meter, for the refusal invariant.
#[derive(Clone, Debug, PartialEq)]
struct MeterState {
    height: u64,
    c_total: Work,
    r_share: Work,
    known_due_reserve: Work,
    overdue_reserve: Work,
    admitted_this_block: Work,
    live: Vec<Object>,
    queue: Vec<Object>,
    terminal: Vec<Object>,
    dated: BTreeMap<u64, Vec<Object>>,
    fault: Fault,
    balance_events: Vec<BalanceEvent>,
    next_id: u64,
    drain_spent_this_block: Work,
}

/// A non-forgeable capability to advance the block height (end the current block). Its only field is
/// private, so it can be constructed ONLY inside this crate, through `Meter::host_tick()`. This models
/// the host/consensus authority to end a block: a contract-level caller, which in a real integration
/// receives only a restricted handle, cannot mint one and so cannot reset its own per-block budget.
pub struct BlockTick(());

/// The meter's ledger and lifecycle collections are PRIVATE so the only way to admit work or advance
/// state is through the checked methods (create/grow/transfer/enqueue/burst/drain), and advancing the
/// block requires the `BlockTick` host capability. If a caller could set `admitted_this_block` back to
/// zero (directly or by forging a block transition), or push a zero-work object straight into
/// `live`/`queue`/`dated`, it would bypass every ingress guard and break the per-block bounds while
/// `check_invariants` still passed. External construction with custom capacities goes through
/// `with_capacities`; in-crate tests access the private fields directly (same crate).
pub struct Meter {
    height: u64,
    /// Block cleanup-and-terminal capacity, partitioned below.
    c_total: Work,
    /// The deadline-free drain rate R (one share of C_total).
    r_share: Work,
    /// The reserved capacity for dated (known_due) work.
    known_due_reserve: Work,
    /// The overdue reserve.
    overdue_reserve: Work,
    /// Positive drain-lane deltas admitted so far this block (the flow ledger).
    admitted_this_block: Work,
    live: Vec<Object>,
    /// Queued deadline-free cleanup items (Discharged objects awaiting drain), FIFO. A VecDeque so the
    /// drain pops the reclaimed prefix in O(1) each and the un-drained tail is never re-inspected.
    queue: VecDeque<Object>,
    /// Objects that have reached their terminal disposition (reclaimed, funding released).
    terminal: Vec<Object>,
    /// Dated work by target height.
    dated: BTreeMap<u64, Vec<Object>>,
    /// The active fault for mutation-checking.
    fault: Fault,
    /// Total balance credited and debited across transfers, for conservation checks.
    balance_events: Vec<BalanceEvent>,
    next_id: u64,
    /// Deadline-free (rate-R) drain work reclaimed so far THIS block, accumulated across every
    /// `drain_block` call and reset only at `end_block`. Persisting it is what bounds the block's
    /// drain-lane reclamation to R even when `drain_block` is called more than once per block.
    drain_spent_this_block: Work,
}

impl Meter {
    /// An illustrative meter sized to exercise the arithmetic. R admits about ten minimal records
    /// of drain work per block; C_total is R plus the two reserves. These are model capacities,
    /// grounded loosely in Phase A's measured record footprint, not platform claims.
    pub fn new() -> Self {
        let r_share = Work::new(2750, 20_000, 200);
        let known_due_reserve = Work::new(2750, 20_000, 200);
        let overdue_reserve = Work::new(1375, 10_000, 100);
        let c_total = r_share.add(known_due_reserve).add(overdue_reserve);
        Meter {
            height: 0,
            c_total,
            r_share,
            known_due_reserve,
            overdue_reserve,
            admitted_this_block: Work::ZERO,
            live: Vec::new(),
            queue: VecDeque::new(),
            terminal: Vec::new(),
            dated: BTreeMap::new(),
            fault: Fault::None,
            balance_events: Vec::new(),
            next_id: 1,
            drain_spent_this_block: Work::ZERO,
        }
    }

    /// Construct a meter with explicit capacities (used by Phase D to size R and the reserves from
    /// measured costs). C_total is the checked sum of the three parts; a sum that overflows u64 is
    /// rejected, since an unrepresentable capacity cannot bound anything. This is the ONLY way an
    /// external caller sets capacities, now that the fields are private.
    pub fn with_capacities(r_share: Work, known_due_reserve: Work, overdue_reserve: Work) -> Self {
        let c_total = r_share
            .checked_add(known_due_reserve)
            .and_then(|p| p.checked_add(overdue_reserve))
            .expect("capacity partition must be representable in u64");
        let mut m = Meter::new();
        m.r_share = r_share;
        m.known_due_reserve = known_due_reserve;
        m.overdue_reserve = overdue_reserve;
        m.c_total = c_total;
        m
    }

    /// Read-only accessor for the current deadline-free drain rate R (Phase D reports it).
    pub fn r_share(&self) -> Work {
        self.r_share
    }

    fn fresh_id(&mut self) -> u64 {
        let id = self.next_id;
        // Checked: never WRAP the id counter (release builds wrap silently), which would mint a
        // duplicate id and break the single-disposition invariant. Exhaustion is fail-closed.
        self.next_id = self.next_id.checked_add(1).expect("object id space exhausted");
        id
    }

    /// The worst-case terminal-work vector for a class, drain-lane portion. Single-owner custody
    /// carries only physical cleanup in the drain lane (its balance discharge is owner-paid).
    /// Autonomous custody carries cleanup plus protocol-driven distribution, scaling with fan-out.
    ///
    /// Returns `None` when the autonomous fan-out computation would overflow u64: a vector that
    /// cannot be represented cannot be a capacity-bearing worst case, so the operation must refuse
    /// rather than admit a silently saturated (understated) vector.
    pub fn class_vector(class: Class, fan_out: u64) -> Option<Work> {
        match class {
            // Cleanup only. Grounded in Phase A: a minimal record's permanent footprint is ~275
            // bytes, its propagation ~2000, hashing ~20.
            Class::SingleOwner => Some(Work::new(275, 2000, 20)),
            // Cleanup plus distribution to `fan_out` recipients, checked so a fan-out that overflows
            // any dimension refuses instead of saturating below its true worst case.
            Class::Autonomous => Some(Work::new(
                275u64.checked_add(178u64.checked_mul(fan_out)?)?,
                2000u64.checked_add(700u64.checked_mul(fan_out)?)?,
                20u64.checked_add(8u64.checked_mul(fan_out)?)?,
            )),
            // A fixed settlement recipe from a pre-materialized finalization record.
            Class::IrrevocableRequest => Some(Work::new(400, 3000, 24)),
        }
    }

    /// Would admitting `delta` keep this block's drain-lane flow at or below R? The flow condition
    /// is `R >= admitted_this_block + delta`, per dimension.
    fn flow_ok(&self, delta: Work) -> bool {
        // Checked: if the prospective total overflows u64 it necessarily exceeds R, so refuse rather
        // than let a saturated total compare as within capacity.
        match self.admitted_this_block.checked_add(delta) {
            Some(total) => self.r_share.ge(total),
            None => false,
        }
    }

    /// CREATE: admit a new position. Its whole vector is a positive delta. Returns the id, or None
    /// if admission is refused by the flow condition.
    pub fn create(&mut self, class: Class, fan_out: u64, funding: u64) -> Option<u64> {
        let drain_work = Self::class_vector(class, fan_out)?; // refuse a fan-out that overflows
        let delta = drain_work; // all new
        if self.fault != Fault::OverAdmitFlow && !self.flow_ok(delta) {
            return None; // refused, nothing admitted
        }
        let accounted = if self.fault == Fault::UnderAccount {
            // Break invariant 1: account less than the worst case.
            Work::new(drain_work.perm.saturating_sub(1), drain_work.prop, drain_work.hash)
        } else {
            drain_work
        };
        let id = self.fresh_id();
        self.admitted_this_block = self.admitted_this_block.add(delta);
        self.live.push(Object {
            id,
            class,
            drain_work,
            accounted,
            funding,
            state: State::Live,
            deadline: None,
            reclaimed: false,
        });
        Some(id)
    }

    /// CREATE with an explicit drain-work vector (used by Phase D to inject Phase C's MEASURED
    /// per-class vectors instead of the illustrative `class_vector`). Admits against the flow
    /// condition; returns None if refused. The object is Live and, like `create`, deadline-free.
    pub fn create_with_vector(&mut self, drain_work: Work, funding: u64) -> Option<u64> {
        // Reject a zero-work object: it would sit in the drain lane consuming a queue slot while
        // adding nothing to `spent`, so an unbounded number could be popped in one block for zero
        // accounted work. Every drain-lane item must carry strictly positive work.
        if drain_work.is_zero() || !self.flow_ok(drain_work) {
            return None;
        }
        self.admitted_this_block = self.admitted_this_block.add(drain_work);
        let id = self.fresh_id();
        self.live.push(Object {
            id,
            class: Class::SingleOwner,
            drain_work,
            accounted: drain_work,
            funding,
            state: State::Live,
            deadline: None,
            reclaimed: false,
        });
        Some(id)
    }

    /// Directly enqueue a deadline-free cleanup item with a measured vector, BYPASSING admission.
    /// Phase D uses this to model an ungoverned backlog (no flow ceiling) so the ceiling's effect
    /// is visible by contrast, and to bulk-load the drain for the mass-retirement load test. Returns
    /// the new id, or None if the vector is zero (a zero-work queue item is rejected, since it would
    /// let an unbounded number be popped in one block without increasing the block's drain spend).
    pub fn enqueue_cleanup(&mut self, drain_work: Work, funding: u64) -> Option<u64> {
        if drain_work.is_zero() {
            return None;
        }
        let id = self.fresh_id();
        self.queue.push_back(Object {
            id,
            class: Class::SingleOwner,
            drain_work,
            accounted: drain_work,
            funding,
            state: State::Discharged,
            deadline: None,
            reclaimed: false,
        });
        Some(id)
    }

    /// The current deadline-free backlog (number of queued cleanup items).
    pub fn backlog(&self) -> usize {
        self.queue.len()
    }

    fn live_mut(&mut self, id: u64) -> Option<&mut Object> {
        self.live.iter_mut().find(|o| o.id == id)
    }

    /// GROW: a footprint-growing mutation. Charges the positive marginal only.
    pub fn grow(&mut self, id: u64, extra: Work) -> bool {
        // Resolve the target first, so a nonexistent id changes nothing (in particular it must not
        // consume flow before discovering the object is missing). Capture its current vectors so the
        // grown totals can be checked for overflow BEFORE any mutation.
        let (cur_drain, cur_acct) = match self.live.iter().find(|o| o.id == id) {
            Some(o) => (o.drain_work, o.accounted),
            None => return false,
        };
        // Checked: the grown drain_work and accounted must be representable. A grow that would
        // overflow u64 is refused rather than saturated (a saturated drain_work would understate the
        // object's true cost and could later drain under a ceiling-high R).
        let new_drain = match cur_drain.checked_add(extra) {
            Some(w) => w,
            None => return false,
        };
        let new_acct = match cur_acct.checked_add(extra) {
            Some(w) => w,
            None => return false,
        };
        // Flow check on the positive delta.
        if self.fault != Fault::OverAdmitFlow && !self.flow_ok(extra) {
            return false;
        }
        self.admitted_this_block = self.admitted_this_block.add(extra);
        let o = self.live_mut(id).expect("resolved above");
        o.drain_work = new_drain;
        o.accounted = new_acct;
        true
    }

    /// RECLASSIFY in place (bidirectional): recompute the vector for the new class and charge every
    /// positive component of the delta. Negative components release nothing transferable.
    pub fn reclassify(&mut self, id: u64, new_class: Class, fan_out: u64) -> bool {
        let old_work = match self.live.iter().find(|o| o.id == id) {
            Some(o) => o.drain_work,
            None => return false,
        };
        let new_work = match Self::class_vector(new_class, fan_out) {
            Some(w) => w,
            None => return false, // refuse a fan-out that overflows
        };
        let positive = new_work.positive_delta(old_work);
        if self.fault != Fault::OverAdmitFlow && !self.flow_ok(positive) {
            return false;
        }
        let fault = self.fault;
        // Under the UnderChargeReclass fault, do NOT admit the positive delta to flow either, so
        // the work moves into the lane entirely uncharged.
        if fault != Fault::UnderChargeReclass {
            self.admitted_this_block = self.admitted_this_block.add(positive);
        }
        if let Some(o) = self.live_mut(id) {
            o.class = new_class;
            o.drain_work = new_work;
            if fault == Fault::UnderChargeReclass {
                // Break invariant 9/1: raise the worst case to the new class but skip the charge,
                // so accounted no longer covers drain_work.
            } else {
                // accounted must cover the new worst case. Charge the positive delta.
                o.accounted = o.accounted.add(positive);
            }
            true
        } else {
            false
        }
    }

    /// PULL: an owner-initiated logical exit that enqueues prepaid cleanup. The object discharges
    /// and its accounting and funding ride with the queued item; nothing is released yet.
    pub fn pull(&mut self, id: u64) -> bool {
        if let Some(pos) = self.live.iter().position(|o| o.id == id) {
            let mut o = self.live.remove(pos);
            o.state = State::Discharged;
            if self.fault == Fault::EarlyRelease {
                // Break invariant 5: release funding at discharge, before reclamation.
                o.funding = 0;
            }
            self.queue.push_back(o);
            true
        } else {
            false
        }
    }

    /// TRANSFER to a new program or environment: DISCHARGE-AND-RECREATE with an accounting handoff.
    /// The old position discharges (its cleanup enqueues, funding and accounting ride along), and a
    /// new position is created with a GROSS positive delta admitted against the current flow. The
    /// balance is debited once and credited once. Returns the new id, or None if refused.
    pub fn transfer(&mut self, id: u64, new_class: Class, fan_out: u64) -> Option<u64> {
        let old_pos = self.live.iter().position(|o| o.id == id)?;
        let new_work = Self::class_vector(new_class, fan_out)?; // refuse a fan-out that overflows
        // GROSS delta: the whole new vector, never netted against the old.
        if self.fault != Fault::OverAdmitFlow && !self.flow_ok(new_work) {
            return None; // refused; old position must be untouched
        }
        // Discharge the old position into the cleanup queue with its accounting and funding intact
        // (funding rides with the queued cleanup item; nothing is released here).
        let mut old = self.live.remove(old_pos);
        old.state = State::Discharged;
        self.queue.push_back(old);
        // Create the new position.
        self.admitted_this_block = self.admitted_this_block.add(new_work);
        let new_id = self.fresh_id();
        self.live.push(Object {
            id: new_id,
            class: new_class,
            drain_work: new_work,
            accounted: new_work,
            // The new position is created with its own prepaid cleanup deposit (nonzero), so that
            // when it is itself terminalized later its funding rides with its queue item, exactly
            // as a freshly created position does. A zero here would falsely read, once pulled, as
            // funding released before reclamation (invariant 5).
            funding: 1,
            state: State::Live,
            deadline: None,
            reclaimed: false,
        });
        // Record the balance move for conservation. The correct path debits once and credits once.
        if self.fault == Fault::TransferDoubleCredit {
            // Break invariant 4: credit the new without debiting the old (double balance).
            self.balance_events.push(BalanceEvent { debited: false, credited: true });
        } else {
            self.balance_events.push(BalanceEvent { debited: true, credited: true });
        }
        Some(new_id)
    }

    /// A transition that must be REFUSED, leaving state bit-identical (invariant 6). Modelled as an
    /// attempt that returns false and changes nothing, unless the MutateOnRefuse fault is active.
    pub fn refuse_and_check_unchanged(&mut self, id: u64) -> bool {
        let before = self.snapshot();
        // The transition is refused (e.g. target is retiring). Under the fault, mutate anyway.
        if self.fault == Fault::MutateOnRefuse {
            if let Some(o) = self.live_mut(id) {
                o.accounted = o.accounted.add(Work::new(1, 0, 0));
            }
        }
        let after = self.snapshot();
        before == after
    }

    /// The known_due share admits at most this many irrevocability transitions targeting a height,
    /// given a per-request settlement vector. A burst beyond the share must fail atomically to
    /// cancelable, producing no dated artifact (invariant 8).
    pub fn irrevocable_burst(&mut self, height: u64, count: u64, per_request: Work) -> bool {
        // Reject a zero-work request (or a zero count): a zero per-request vector passes every reserve
        // check and would let an arbitrary `count` be materialized (Vec::with_capacity(count)) and
        // completed for zero accounted work. Every dated item must carry strictly positive work.
        if per_request.is_zero() || count == 0 {
            return false;
        }
        // A deadline must be reserved strictly IN ADVANCE: refuse a target height at or before the
        // current one. Rejecting the current height is what makes "one reserve per block" hold: a
        // caller cannot drain the current height's bucket and then refill the same height in the same
        // block (which, since the combined-reservation check reads the now-empty bucket, would let a
        // second full reserve drain in the same block). Reserving is always for a future height.
        if height <= self.height {
            return false;
        }
        // Capacity of the known_due reserve in units of per_request (perm dimension as the binding
        // one for the model).
        let share_capacity = if per_request.perm == 0 {
            u64::MAX
        } else {
            self.known_due_reserve.perm / per_request.perm
        };
        let admit = self.fault == Fault::OverIrrevocable || count <= share_capacity;
        if !admit {
            // Fail atomically: no artifact enqueued, escrow intact.
            return false;
        }
        // Checked: a burst whose total work overflows u64 cannot fit the reserve, so refuse rather
        // than let a saturated total pass the reserve check.
        let total = match per_request.checked_mul_scalar(count) {
            Some(t) => t,
            None => return false,
        };
        // Account for work ALREADY reserved at this height: the known_due reserve bounds the TOTAL
        // dated work at a height, not each burst in isolation, so two bursts that each fit the reserve
        // but together exceed it must not both be admitted.
        let existing = self
            .dated
            .get(&height)
            .map(|items| {
                items
                    .iter()
                    .try_fold(Work::ZERO, |acc, o| acc.checked_add(o.drain_work))
            })
            .unwrap_or(Some(Work::ZERO));
        let combined = match existing.and_then(|e| e.checked_add(total)) {
            Some(c) => c,
            None => return false,
        };
        // Only admit if the combined reserved work fits the known_due reserve (unless the fault forces
        // it). The fault path admits beyond the reserve, which the dated-bucket invariant then flags.
        if self.fault != Fault::OverIrrevocable && !self.known_due_reserve.ge(combined) {
            return false;
        }
        let mut objs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let id = self.fresh_id();
            objs.push(Object {
                id,
                class: Class::IrrevocableRequest,
                drain_work: per_request,
                accounted: per_request,
                funding: 1,
                state: State::Live,
                deadline: Some(height),
                reclaimed: false,
            });
        }
        self.dated.entry(height).or_default().extend(objs);
        true
    }

    /// Process the deadline-free drain for one block: reclaim queued cleanup items up to the R
    /// share (and never more than the one-fifth cap, modelled here as R itself for the drain lane).
    /// A reclaimed item becomes Terminal and releases its funding. Dated work due at EXACTLY the
    /// current height is completed first (invariant 7). Only the current height's bucket is processed
    /// per block, so one block completes at most one height's reserved work (bounded by the known_due
    /// reserve via inv8). A bucket left unprocessed past its height is overdue and is flagged by the
    /// overdue invariant rather than being swept up later in a single over-capacity catch-up.
    pub fn drain_block(&mut self) {
        // 1. Complete dated work due at exactly this height (no multi-height catch-up). Look the
        // current height's bucket up DIRECTLY (O(log N)) rather than iterating every future bucket, so
        // a large number of pending future heights does not make each block do work unrelated to it.
        if self.fault != Fault::MissDeadline {
            // Completing dated work moves each item through the SAME explicit terminal transition as
            // a drained queue item (reclaimed, Terminal, funding released), so the release-timing and
            // single-disposition invariants can observe it, rather than dropping it silently.
            if let Some(items) = self.dated.remove(&self.height) {
                for mut o in items {
                    o.reclaimed = true;
                    o.state = State::Terminal;
                    o.funding = 0;
                    self.terminal.push(o);
                }
            }
        }
        // 2. Drain the deadline-free queue up to R, componentwise across every resource dimension.
        // This is a FIFO rate-limited drain: reclaim a PREFIX of the queue, stopping at the first item
        // that does not fit this block's remaining R in every dimension. An item is reclaimed only if
        // adding its full work vector keeps the block's spend at or below R in EVERY dimension, so an
        // item whose propagation or hashing exceeds the reserved rate is not reclaimed on a small perm
        // cost alone; it (and everything behind it) stays queued until a later block. Spend ACCUMULATES
        // across every drain_block call within a block (reset only at end_block), so calling
        // drain_block twice at the same height cannot reclaim two full R shares.
        //
        // Because we stop at the first non-fitting front item, per-block work is bounded by the number
        // reclaimed (pop_front is O(1) on the VecDeque), so the whole backlog drains in O(N) total, not
        // O(N^2): no already-scanned tail is re-inspected each block. (A production scheduler that
        // needed OUT-OF-ORDER packing would carry a cursor and a per-block inspection budget instead;
        // this prototype uses the simpler FIFO-prefix discipline, which every current scenario uses.)
        let mut spent = self.drain_spent_this_block;
        while let Some(front) = self.queue.front() {
            // Checked: an item whose addition would overflow the accumulated spend cannot fit within R.
            let fits = match spent.checked_add(front.drain_work) {
                Some(after) => self.r_share.ge(after),
                None => false,
            };
            if !fits {
                break; // FIFO: the first non-fitting item stops the block, tail stays in place.
            }
            let mut o = self.queue.pop_front().expect("front exists");
            spent = spent.checked_add(o.drain_work).expect("fit checked above");
            o.reclaimed = true;
            o.state = State::Terminal;
            // Release funding only now, at reclamation (invariant 5).
            o.funding = 0;
            self.terminal.push(o);
        }
        // Persist the accumulated drain spend for the rest of this block.
        self.drain_spent_this_block = spent;
    }

    /// Mass retirement: freeze all live objects and drive each to exactly one terminal disposition.
    /// A user's exit must not change another user's rights (invariant 10). Modelled by moving every
    /// live object to the cleanup queue with a single terminal path.
    pub fn mass_retire(&mut self) {
        let live = std::mem::take(&mut self.live);
        for mut o in live {
            o.state = State::Discharged;
            self.queue.push_back(o.clone());
            if self.fault == Fault::DoubleTerminal {
                // Break invariant 10: also file the same object under a second terminal path.
                self.queue.push_back(o);
            }
        }
    }

    /// Advance to the next block, resetting the per-block flow ledger AND the per-block drain spend,
    /// so the drain-lane reclamation budget refreshes exactly once per block.
    ///
    /// Gated behind a `BlockTick` CAPABILITY so the per-block budget cannot be reset by an untrusted
    /// caller. Advancing the block is a HOST action (in a real deployment the consensus layer decides
    /// when a block ends); a contract-level caller has no way to mint a `BlockTick`, so it cannot
    /// forge a block transition to hand itself a fresh R within one authoritative height. Each tick
    /// advances exactly one block (it is consumed by value). The prototype's driver (Phase D, tests)
    /// plays the host and mints the ticks.
    pub fn end_block(&mut self, _tick: BlockTick) {
        // Checked: never WRAP the height (release builds wrap silently), which would break deadline
        // ordering and return the model to height zero. Exhaustion is fail-closed.
        self.height = self.height.checked_add(1).expect("block height space exhausted");
        self.admitted_this_block = Work::ZERO;
        self.drain_spent_this_block = Work::ZERO;
    }

    /// Mint a block-advance capability. This models HOST authority: in production the consensus/host
    /// layer holds the ability to end a block, never a contract. Kept as the single mint point so the
    /// capability's origin is explicit.
    pub fn host_tick() -> BlockTick {
        BlockTick(())
    }

    /// A hashable snapshot of mutable state, for the refusal invariant.
    /// A comprehensive snapshot of ALL mutable state, including the capacity partition and the fault,
    /// so the refusal invariant detects ANY change. This is what makes invariant 6 (a refused
    /// transition leaves state bit-identical) actually load-bearing.
    fn snapshot(&self) -> MeterState {
        MeterState {
            height: self.height,
            c_total: self.c_total,
            r_share: self.r_share,
            known_due_reserve: self.known_due_reserve,
            overdue_reserve: self.overdue_reserve,
            admitted_this_block: self.admitted_this_block,
            live: self.live.clone(),
            queue: self.queue.iter().cloned().collect(),
            terminal: self.terminal.clone(),
            dated: self.dated.clone(),
            fault: self.fault,
            balance_events: self.balance_events.clone(),
            next_id: self.next_id,
            drain_spent_this_block: self.drain_spent_this_block,
        }
    }

    /// Check every invariant that is a continuous property of state. Returns the first violation.
    pub fn check_invariants(&self) -> Result<(), String> {
        // Invariant 1: accounted >= worst_case_work for every live object, every queued item, AND
        // every dated (deadline-bearing) item, so no accounting-bearing object escapes the check.
        for o in self
            .live
            .iter()
            .chain(self.queue.iter())
            .chain(self.dated.values().flatten())
        {
            if !o.accounted.ge(o.drain_work) {
                return Err(format!(
                    "inv1 accounting: object {} accounted {:?} < drain_work {:?}",
                    o.id, o.accounted, o.drain_work
                ));
            }
        }
        // Invariant 2: admitted positive drain-lane deltas this block <= R.
        if !self.r_share.ge(self.admitted_this_block) {
            return Err(format!(
                "inv2 flow: admitted {:?} exceeds R {:?}",
                self.admitted_this_block, self.r_share
            ));
        }
        // Invariant 3: known_due + R + overdue <= C_total. Checked: a partition sum that overflows
        // u64 cannot fit C_total, so it is an invariant violation, not a saturated pass.
        let partition = match self
            .known_due_reserve
            .checked_add(self.r_share)
            .and_then(|p| p.checked_add(self.overdue_reserve))
        {
            Some(p) => p,
            None => {
                return Err(format!(
                    "inv3 partition: known_due {:?} + R {:?} + overdue {:?} overflows u64",
                    self.known_due_reserve, self.r_share, self.overdue_reserve
                ))
            }
        };
        if !self.c_total.ge(partition) {
            return Err(format!(
                "inv3 partition: known_due+R+overdue {:?} exceeds C_total {:?}",
                partition, self.c_total
            ));
        }
        // Invariant 7: no dated bucket is OVERDUE. A bucket whose deadline height is strictly below
        // the current height was not completed at its deadline (a missed deadline, e.g. the
        // MissDeadline fault or a skipped drain), so it is flagged here rather than being swept up in
        // a later over-capacity catch-up.
        for h in self.dated.keys() {
            if *h < self.height {
                return Err(format!(
                    "inv7 overdue: dated work at height {} is overdue at current height {}",
                    h, self.height
                ));
            }
        }
        // Invariant 8: the ACTUAL dated work reserved at each height fits the known_due reserve. This
        // checks the reserved buckets themselves (not just each burst at admission), so accumulated
        // over-reservation at a height (or an OverIrrevocable fault) is caught here.
        for (h, items) in self.dated.iter() {
            let reserved = items
                .iter()
                .try_fold(Work::ZERO, |acc, o| acc.checked_add(o.drain_work));
            match reserved {
                Some(r) if self.known_due_reserve.ge(r) => {}
                Some(r) => {
                    return Err(format!(
                        "inv8 reserve: dated work {:?} at height {} exceeds known_due reserve {:?}",
                        r, h, self.known_due_reserve
                    ))
                }
                None => {
                    return Err(format!(
                        "inv8 reserve: dated work at height {h} overflows u64"
                    ))
                }
            }
        }
        // Invariant 4: every balance move debits once and credits once.
        for (i, e) in self.balance_events.iter().enumerate() {
            if e.credited != e.debited {
                return Err(format!(
                    "inv4 conservation: balance event {} credited={} debited={}",
                    i, e.credited, e.debited
                ));
            }
        }
        // Invariant 5: no Discharged (queued, not yet reclaimed) object has had its funding
        // released early, and every Terminal (reclaimed) object HAS released its funding. The
        // release happens at reclamation, neither before nor never.
        for o in self.queue.iter() {
            if o.state == State::Discharged && !o.reclaimed && o.funding == 0 {
                return Err(format!(
                    "inv5 release-timing: queued object {} released funding before reclamation",
                    o.id
                ));
            }
        }
        for o in self.terminal.iter() {
            if o.funding != 0 {
                return Err(format!(
                    "inv5 release-timing: terminal object {} retains funding after reclamation",
                    o.id
                ));
            }
        }
        // Invariant 10: no object id appears twice across ALL disposition-bearing collections (live,
        // queued, dated, and terminal), so a duplicate cannot hide by draining out of the queue.
        let mut seen = std::collections::HashSet::new();
        for o in self
            .live
            .iter()
            .chain(self.queue.iter())
            .chain(self.dated.values().flatten())
            .chain(self.terminal.iter())
        {
            if !seen.insert(o.id) {
                return Err(format!(
                    "inv10 disposition: object {} reachable in two dispositions",
                    o.id
                ));
            }
        }
        Ok(())
    }
}

impl Default for Meter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each invariant gets two checks: it PASSES on the clean path, and it FAILS under the fault
    // built to break it. The failing direction is the mutation-check ("a test does not exist until
    // it has been watched failing").

    #[test]
    fn inv1_accounting_holds_and_breaks() {
        // Clean: create some objects, accounting covers worst case.
        let mut m = Meter::new();
        m.create(Class::SingleOwner, 0, 100);
        m.create(Class::Autonomous, 3, 100);
        assert!(m.check_invariants().is_ok());

        // Broken: under-account on create.
        let mut m = Meter::new();
        m.fault = Fault::UnderAccount;
        m.create(Class::SingleOwner, 0, 100);
        assert!(m.check_invariants().unwrap_err().contains("inv1"));
    }

    #[test]
    fn inv2_flow_holds_and_breaks() {
        // Clean: admit within R.
        let mut m = Meter::new();
        for _ in 0..5 {
            m.create(Class::SingleOwner, 0, 10);
        }
        assert!(m.check_invariants().is_ok());
        assert!(m.r_share.ge(m.admitted_this_block));

        // Broken: force admission beyond R in one block.
        let mut m = Meter::new();
        m.fault = Fault::OverAdmitFlow;
        for _ in 0..100 {
            m.create(Class::SingleOwner, 0, 10);
        }
        assert!(m.check_invariants().unwrap_err().contains("inv2"));
    }

    #[test]
    fn inv3_partition_holds_and_breaks() {
        let m = Meter::new();
        assert!(m.check_invariants().is_ok());

        let mut m = Meter::new();
        // Break the partition directly: inflate a reserve past C_total.
        m.known_due_reserve = m.c_total;
        assert!(m.check_invariants().unwrap_err().contains("inv3"));
    }

    #[test]
    fn capacity_decisions_refuse_at_the_u64_ceiling() {
        // Admission: with the flow ledger already at the ceiling, one more unit overflows u64 and is
        // refused, rather than saturating to u64::MAX and comparing as within an R of u64::MAX.
        let mut m = Meter::new();
        m.r_share = Work::new(u64::MAX, u64::MAX, u64::MAX);
        m.admitted_this_block = Work::new(u64::MAX, u64::MAX, u64::MAX);
        assert!(
            !m.flow_ok(Work::new(1, 0, 0)),
            "an overflowing admission is refused, not saturated through a ceiling-high R"
        );

        // Partition: reserves that sum past u64 are flagged as an invariant violation, not saturated
        // into fitting a ceiling-high C_total.
        let mut m = Meter::new();
        m.c_total = Work::new(u64::MAX, u64::MAX, u64::MAX);
        m.known_due_reserve = Work::new(u64::MAX, 0, 0);
        m.r_share = Work::new(2, 0, 0);
        m.overdue_reserve = Work::ZERO;
        assert!(
            m.check_invariants().unwrap_err().contains("inv3"),
            "an overflowing partition is flagged, not saturated into a pass"
        );

        // Burst: a per-request-times-count whose non-binding dimension overflows u64 is refused, even
        // when it passes the perm-dimension share-capacity gate.
        let mut m = Meter::new();
        m.known_due_reserve = Work::new(u64::MAX, u64::MAX, u64::MAX);
        assert!(
            !m.irrevocable_burst(100, 2, Work::new(1, u64::MAX, 1)),
            "an overflowing burst total is refused, not saturated into a reserve that appears to fit"
        );
    }

    #[test]
    fn overflowing_fan_out_and_grow_are_refused() {
        // An autonomous fan-out whose vector overflows u64 is refused at create (class_vector returns
        // None), rather than admitting a saturated, understated worst case.
        let mut m = Meter::new();
        m.r_share = Work::new(u64::MAX, u64::MAX, u64::MAX);
        assert!(
            m.create(Class::Autonomous, u64::MAX, 1).is_none(),
            "an overflowing fan-out is refused at create"
        );

        // A grow that would overflow the object's drain_work is refused, leaving the object unchanged,
        // so a saturated drain_work can never later drain under a ceiling-high R.
        let mut m = Meter::new();
        m.r_share = Work::new(u64::MAX, u64::MAX, u64::MAX);
        let id = m.create(Class::SingleOwner, 0, 1).expect("create");
        m.live[0].drain_work = Work::new(u64::MAX, 0, 0);
        m.live[0].accounted = Work::new(u64::MAX, 0, 0);
        let before = m.snapshot();
        assert!(!m.grow(id, Work::new(1, 0, 0)), "a grow that overflows drain_work is refused");
        assert_eq!(m.snapshot(), before, "a refused grow changes nothing");
    }

    #[test]
    fn irrevocable_burst_accounts_work_already_reserved_at_a_height() {
        // The known_due reserve bounds TOTAL dated work at a height, not each burst alone.
        let per = Work::new(400, 3000, 24); // default reserve = (2750, 20000, 200); share_capacity = 6
        let mut m = Meter::new();
        assert!(m.irrevocable_burst(5, 6, per), "the first burst of 6 fits the reserve");
        assert!(
            !m.irrevocable_burst(5, 6, per),
            "a second burst at the same height that would exceed the combined reserve is refused"
        );
        assert_eq!(
            m.dated.get(&5).map(|v| v.len()),
            Some(6),
            "the refused second burst produced no dated artifact"
        );
        assert!(m.check_invariants().is_ok(), "the reserved bucket satisfies inv8");

        // The dated-bucket invariant itself catches over-reservation (here forced via the fault).
        let mut m = Meter::new();
        m.fault = Fault::OverIrrevocable;
        assert!(m.irrevocable_burst(5, 100, per), "the faulted gate admits beyond the reserve");
        assert!(
            m.check_invariants().unwrap_err().contains("inv8"),
            "the dated-bucket invariant flags reserved work beyond the known_due reserve"
        );
    }

    #[test]
    fn inv4_transfer_conservation_holds_and_breaks() {
        // Clean: a transfer debits once and credits once.
        let mut m = Meter::new();
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        m.transfer(id, Class::Autonomous, 2);
        assert!(m.check_invariants().is_ok());

        // Broken: credit without debit.
        let mut m = Meter::new();
        m.fault = Fault::TransferDoubleCredit;
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        m.transfer(id, Class::Autonomous, 2);
        assert!(m.check_invariants().unwrap_err().contains("inv4"));
    }

    #[test]
    fn inv5_release_timing_holds_and_breaks() {
        // Clean: pull enqueues cleanup, funding rides along until drain reclaims it.
        let mut m = Meter::new();
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        m.pull(id);
        assert!(m.check_invariants().is_ok(), "funding must ride with the queued item");
        // After drain, the item is terminal and funding released; still consistent.
        m.drain_block();
        assert!(m.check_invariants().is_ok());

        // Broken: release funding at discharge, before reclamation.
        let mut m = Meter::new();
        m.fault = Fault::EarlyRelease;
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        m.pull(id);
        assert!(m.check_invariants().unwrap_err().contains("inv5"));
    }

    #[test]
    fn inv6_refusal_leaves_state_unchanged_and_breaks() {
        // Clean: a refused transition changes nothing.
        let mut m = Meter::new();
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        assert!(m.refuse_and_check_unchanged(id), "refusal must leave state bit-identical");

        // Broken: mutate on refusal.
        let mut m = Meter::new();
        m.fault = Fault::MutateOnRefuse;
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        assert!(!m.refuse_and_check_unchanged(id), "faulted refusal must be detected as a change");
    }

    #[test]
    fn inv7_dated_work_meets_deadline_and_breaks() {
        // Clean: dated work due at height H is completed by the drain at H.
        let mut m = Meter::new();
        m.irrevocable_burst(1, 2, Work::new(400, 3000, 24));
        m.end_block(Meter::host_tick()); // height 1
        m.drain_block();
        assert!(m.dated.is_empty(), "dated work due at height 1 must be completed");

        // Broken: the drain skips due dated work, leaving the deadline missed; once the block
        // advances past that height, the overdue invariant flags the stranded bucket.
        let mut m = Meter::new();
        m.fault = Fault::MissDeadline;
        m.irrevocable_burst(1, 2, Work::new(400, 3000, 24));
        m.end_block(Meter::host_tick()); // height 1
        m.drain_block();
        assert!(!m.dated.is_empty(), "faulted drain must leave the deadline missed");
        m.end_block(Meter::host_tick()); // height 2: the height-1 bucket is now overdue
        assert!(
            m.check_invariants().unwrap_err().contains("inv7"),
            "an overdue dated bucket (missed deadline) is flagged"
        );
    }

    #[test]
    fn dated_drain_completes_only_the_current_height() {
        // Two full buckets reserved at future heights 1 and 2. Each block completes at most its own
        // height's bucket, so one block never completes two heights' reserved work at once.
        let per = Work::new(400, 3000, 24); // reserve (2750, 20000, 200), share 6
        let mut m = Meter::new();
        assert!(m.irrevocable_burst(1, 6, per));
        assert!(m.irrevocable_burst(2, 6, per));
        m.end_block(Meter::host_tick()); // height 1
        m.drain_block();
        assert_eq!(m.dated.get(&1), None, "the height-1 bucket completed at height 1");
        assert_eq!(m.dated.get(&2).map(|v| v.len()), Some(6), "the height-2 bucket is not yet due");
        assert!(m.check_invariants().is_ok());
        m.end_block(Meter::host_tick()); // height 2
        m.drain_block();
        assert!(m.dated.is_empty(), "the height-2 bucket completed at its own block");
        assert!(m.check_invariants().is_ok());
    }

    #[test]
    fn irrevocable_burst_refuses_a_past_or_current_deadline() {
        let per = Work::new(400, 3000, 24);
        let mut m = Meter::new();
        m.end_block(Meter::host_tick());
        m.end_block(Meter::host_tick()); // height 2
        assert!(
            !m.irrevocable_burst(1, 1, per),
            "a burst targeting a height already in the past is refused"
        );
        assert!(
            !m.irrevocable_burst(2, 1, per),
            "a burst targeting the CURRENT height is refused (reservation must be in advance)"
        );
        assert!(m.dated.is_empty(), "a refused burst produces no dated artifact");
    }

    #[test]
    fn one_dated_reserve_per_block_no_same_height_refill() {
        // The whole point of requiring a future deadline: a caller cannot drain the current height's
        // bucket and then refill the same height to drain a SECOND full reserve in the same block.
        let per = Work::new(400, 3000, 24);
        let mut m = Meter::new();
        assert!(m.irrevocable_burst(1, 6, per), "reserve a full bucket for height 1");
        m.end_block(Meter::host_tick()); // height 1
        m.drain_block(); // completes the height-1 bucket (one reserve)
        assert!(m.dated.is_empty(), "the height-1 bucket completed");
        // Refilling the current height is refused, so a second reserve cannot drain this same block.
        assert!(
            !m.irrevocable_burst(1, 6, per),
            "cannot reserve for the current height after draining it (no same-block second reserve)"
        );
        m.drain_block(); // nothing left to complete
        assert!(m.dated.is_empty());
        assert!(m.check_invariants().is_ok());
    }

    #[test]
    fn drain_spend_accumulates_across_calls_within_a_block() {
        // The deadline-free drain budget R is per BLOCK, not per drain_block call: calling drain_block
        // twice at the same height must not reclaim two full R shares.
        let mut m = Meter::new(); // R = (2750, 20000, 200); a minimal item is (275, 2000, 20) => 10 fit
        for _ in 0..20 {
            m.enqueue_cleanup(Work::new(275, 2000, 20), 1);
        }
        assert_eq!(m.backlog(), 20);
        m.drain_block();
        let after_first = m.backlog();
        m.drain_block(); // SAME block, no end_block
        let after_second = m.backlog();
        assert_eq!(
            after_second, after_first,
            "a second drain at the same height reclaims nothing more (R already spent this block)"
        );
        let reclaimed = 20 - after_second;
        assert!(
            reclaimed <= 10,
            "combined reclamation across two same-block drains stays within one R share (got {reclaimed})"
        );
        assert!(m.check_invariants().is_ok());
        // After end_block the budget refreshes, so the next block drains a fresh R share.
        m.end_block(Meter::host_tick());
        m.drain_block();
        assert!(m.backlog() < after_second, "the next block drains a fresh R share");
    }

    #[test]
    fn inv8_irrevocable_burst_gate_holds_and_breaks() {
        // Clean: a burst beyond the known_due share fails atomically, no artifact.
        let mut m = Meter::new();
        let per = Work::new(400, 3000, 24);
        let capacity = m.known_due_reserve.perm / per.perm;
        let admitted = m.irrevocable_burst(1, capacity + 5, per);
        assert!(!admitted, "over-share burst must be refused");
        assert!(m.dated.is_empty(), "refused burst must produce no dated artifact");

        // Broken: admit the over-share burst anyway.
        let mut m = Meter::new();
        m.fault = Fault::OverIrrevocable;
        let admitted = m.irrevocable_burst(1, capacity + 5, per);
        assert!(admitted && !m.dated.is_empty(), "faulted gate admits beyond the share");
    }

    #[test]
    fn inv9_reclassification_charges_positive_only_and_breaks() {
        // Clean: reclassifying up charges the positive delta; accounting still covers worst case.
        let mut m = Meter::new();
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        m.reclassify(id, Class::Autonomous, 3);
        assert!(m.check_invariants().is_ok());
        // Reclassifying back down must not create spendable credit.
        m.reclassify(id, Class::SingleOwner, 0);
        assert!(m.check_invariants().is_ok());

        // Broken: an upward reclassification raises the worst case but skips the charge, so the
        // class change moves work into the lane uncharged (accounted < worst case).
        let mut m = Meter::new();
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        m.fault = Fault::UnderChargeReclass;
        m.reclassify(id, Class::Autonomous, 3);
        assert!(m.check_invariants().unwrap_err().contains("inv1"),
            "an uncharged upward reclassification must drop accounted below worst case (inv1)");
    }

    #[test]
    fn inv10_mass_retirement_single_disposition_and_breaks() {
        // Clean: every retired object reaches exactly one terminal disposition.
        let mut m = Meter::new();
        for _ in 0..5 {
            m.create(Class::SingleOwner, 0, 10);
        }
        m.mass_retire();
        assert!(m.check_invariants().is_ok());

        // Broken: file an object under two dispositions.
        let mut m = Meter::new();
        m.fault = Fault::DoubleTerminal;
        m.create(Class::SingleOwner, 0, 10);
        m.mass_retire();
        assert!(m.check_invariants().unwrap_err().contains("inv10"));
    }

    #[test]
    fn grow_on_missing_object_consumes_no_flow() {
        // Regression: grow on a nonexistent id must change nothing, in particular it must not
        // consume block flow before discovering the object is missing.
        let mut m = Meter::new();
        let before = m.admitted_this_block;
        assert!(!m.grow(999, Work::new(1, 0, 0)), "grow on a missing id fails");
        assert_eq!(m.admitted_this_block, before, "a failed grow consumes no flow");
        assert!(m.check_invariants().is_ok());
    }

    #[test]
    fn drain_respects_every_dimension() {
        // Regression: an item whose propagation exceeds the reserved rate must not be reclaimed on
        // the strength of a small perm cost. Enqueue an item with prop above r_share.prop and a
        // tiny perm, and confirm the drain leaves it queued.
        let mut m = Meter::new();
        let heavy_prop = Work::new(1, m.r_share.prop + 1, 1);
        m.enqueue_cleanup(heavy_prop, 1);
        m.drain_block();
        assert_eq!(m.backlog(), 1, "an item exceeding R in the propagation dimension is not drained");
        assert!(m.check_invariants().is_ok());
    }

    #[test]
    fn zero_work_items_are_rejected_at_every_ingress() {
        // A zero-work item would let an unbounded number be popped/completed in one block for zero
        // accounted work, defeating the per-block bound. Every drain-lane and dated ingress rejects it.
        let mut m = Meter::new();
        assert!(m.create_with_vector(Work::ZERO, 1).is_none(), "a zero-work create is refused");
        assert!(m.enqueue_cleanup(Work::ZERO, 1).is_none(), "a zero-work cleanup enqueue is refused");
        assert_eq!(m.backlog(), 0, "no zero-work item entered the queue");
        assert!(
            !m.irrevocable_burst(5, 1_000_000, Work::ZERO),
            "a zero-work irrevocable burst is refused (no arbitrary-count allocation)"
        );
        assert!(m.dated.is_empty(), "no zero-work dated artifact was created");
        assert!(m.check_invariants().is_ok());
    }

    #[test]
    fn drain_is_fifo_prefix_and_stops_at_a_non_fitting_front() {
        // The drain reclaims a FIFO prefix and stops at the first non-fitting item, so a heavy front
        // item blocks the queue (head-of-line) and per-block work is bounded by what is reclaimed,
        // rather than scanning the whole backlog every block.
        let mut m = Meter::new();
        // A heavy front item that exceeds R in the propagation dimension, then many small items.
        m.enqueue_cleanup(Work::new(1, m.r_share.prop + 1, 1), 1);
        for _ in 0..1000 {
            m.enqueue_cleanup(Work::new(275, 2000, 20), 1);
        }
        let before = m.backlog();
        m.drain_block();
        assert_eq!(
            m.backlog(),
            before,
            "a non-fitting front item stops the block; nothing behind it drains (FIFO head-of-line)"
        );
        assert!(m.check_invariants().is_ok());
    }

    #[test]
    fn transfer_then_pull_is_funded() {
        // Regression: a transferred position is created with its own cleanup deposit, so pulling it
        // does not read as funding released before reclamation (invariant 5).
        let mut m = Meter::new();
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        let new_id = m.transfer(id, Class::Autonomous, 1).unwrap();
        m.pull(new_id);
        assert!(m.check_invariants().is_ok(), "a pulled transferred position is funded");
    }

    #[test]
    fn real_refusals_leave_state_unchanged() {
        // Exercise REAL refused operations (not a synthetic no-op) against the comprehensive
        // snapshot: each must leave every field of the meter bit-identical.
        let mut m = Meter::new();
        // Fill the flow ledger so further admissions are refused.
        while m.create(Class::SingleOwner, 0, 1).is_some() {}
        let id = m.live[0].id;

        let before = m.snapshot();
        assert!(m.create(Class::SingleOwner, 0, 1).is_none(), "create refused when flow is exhausted");
        assert_eq!(m.snapshot(), before, "a refused create changes nothing");

        let before = m.snapshot();
        assert!(!m.grow(999_999, Work::new(1, 0, 0)), "grow on a missing id is refused");
        assert_eq!(m.snapshot(), before, "a refused grow changes nothing");

        let before = m.snapshot();
        assert!(m.transfer(id, Class::Autonomous, 1).is_none(), "transfer refused when flow exhausted");
        assert_eq!(m.snapshot(), before, "a refused transfer changes nothing");

        let before = m.snapshot();
        assert!(!m.reclassify(id, Class::Autonomous, 8), "reclassify refused when flow exhausted");
        assert_eq!(m.snapshot(), before, "a refused reclassify changes nothing");
    }

    #[test]
    fn certified_scenario_transfer_chain_stays_bounded() {
        // A chain of transfers must consume flow equal to each new vector (gross delta), never
        // recycling one admission, and every queued cleanup item stays accounted. Run several
        // transfers across blocks and confirm invariants hold throughout.
        let mut m = Meter::new();
        let mut id = m.create(Class::SingleOwner, 0, 100).unwrap();
        for _ in 0..10 {
            m.end_block(Meter::host_tick()); // fresh flow each block
            if let Some(new_id) = m.transfer(id, Class::Autonomous, 1) {
                id = new_id;
            }
            m.drain_block();
            assert!(m.check_invariants().is_ok(), "invariants hold across the transfer chain");
        }
    }

    #[test]
    fn certified_scenario_equal_aggregate_different_lane_reclassification() {
        // The round-12 case: a reclassification whose aggregate is unchanged but whose lane
        // attribution differs must still charge the positive destination-lane components. Modelled
        // by reclassifying single->autonomous with a fan-out chosen so perm grows even if another
        // notion of "aggregate" might not; the positive-delta rule must admit the growth.
        let mut m = Meter::new();
        let id = m.create(Class::SingleOwner, 0, 100).unwrap();
        let before = m.admitted_this_block;
        m.reclassify(id, Class::Autonomous, 2);
        assert!(!m.admitted_this_block.positive_delta(before).is_zero(),
            "a growing reclassification must admit positive flow, not read as zero-growth");
        assert!(m.check_invariants().is_ok());
    }
}
