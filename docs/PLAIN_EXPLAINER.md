# A plain explainer of the Dash Platform execution-layer project

This document explains, in ordinary language, what this project is, why it exists, how the work is done,
and what the design currently says. It is written for a reader who is not a blockchain engineer.
It
defines each technical term the first time it appears and leans on analogies. The precise, formal
version
of everything here lives in `DESIGN.md`, the review evidence in the project's review record, and the measurement
work in `docs/PHASE0_FINDINGS.md`. This file is the friendly companion to those, not a replacement.

Written 2026-08-09 at version 9. Updated 2026-08-10 when the review loop CLOSED at version 12 (round 12
returned a clean approval from every reviewing family). Updated 2026-08-11 with Part 7 (the metering
prototype that measured the design's open numbers) and the Ethereum part. Updated 2026-08-12 with a new
Part 8 (adopting the CosmWasm program-runner instead of building one, now shown by running spikes) and a
revised Part 9 (the Ethereum-as-a-guest shape, now demonstrated by a running spike rather than only
argued). Part 6 tells the review's ending in full. The headline is unchanged in kind. The design is
review-complete ON PAPER, meaning the independent reviewers found nothing left to fix in the text, its
cost model has since been measured against the real storage engine, and a candidate way to gain the
compute ability by adopting an existing runner has now been demonstrated to work in small spikes, but
nothing is "closed" in the strong sense until an implementation exists and is tested against it.

## The one-paragraph version

Dash Platform is a system that can reliably store data, look it up, prove to an outsider that a stored
fact is true, and check that whoever is changing the data is allowed to. What it cannot do today is
COMPUTE: it cannot run a program that takes some inputs, does arithmetic or logic, and writes the result
back, in a way that every participant is guaranteed to agree on. This project asks a single question,
what would it take to add that ability safely, and works out the answer as a design on paper. It is not
being built. I am pressure-testing the design by having several independent reviewers attack it in
rounds, fixing what they break, and repeating until it stops breaking.

## Part 1. The background, and the gap to fill

### What a blockchain is, briefly

A blockchain is a shared record book that many independent computers keep identical copies of. No single
computer is in charge. They agree on what the record says by following a fixed procedure, so that a
newcomer can join, download the record, and trust it without trusting any one operator. The value of
this
arrangement is that it removes the need for a trusted middleman. The cost is that everything is harder,
because every participant must be able to independently reach the exact same answer, or the shared
record
splits into disagreeing versions, which defeats the whole point.

Dash is one such blockchain, originally focused on payments. Dash Platform is a second layer built on
top
of it that stores richer application data, not just balances. Think of the base Dash chain as a bank
ledger of who owns how much, and Dash Platform as a set of application databases sitting beside it, for
things like usernames, contact lists, and app-specific records.

### What Dash Platform can already do, and the one thing it cannot

Dash Platform is good at four things:

- STORE. It keeps structured data, organised into what it calls data contracts (roughly, database
  schemas) and documents (roughly, database rows).
-  INDEX. It can look data up quickly by more than one key, the way a library catalog lets you find a
  book
  by title or by author.
- PROVE. This is the special one. It can hand a lightweight client a small cryptographic proof that a
  particular fact is in the record, or is NOT in the record, without the client downloading everything.
  Cryptographic here means the proof relies on hard mathematics rather than on trusting the sender.
  Imagine a librarian who can prove a specific book is on the shelf by showing you a tamper-evident
  receipt, instead of making you walk the stacks.
-  AUTHORIZE. It checks signatures, so only the rightful owner can change their own data. A signature
  here
  is a piece of math only the holder of a secret key can produce, and anyone can verify.

The missing fifth ability is COMPUTE. Today the rules for changing data are fixed and built into the
platform itself. You cannot upload your own program that says "when someone sends me tokens, run this
logic and update these balances." The platform can hold your data and prove it, but it cannot run your
arithmetic. The analogy is a filing cabinet with a notary attached, excellent at keeping documents and
certifying them, but with no calculator inside, and with no way to leave instructions for it to follow
on
its own.

### Why anyone would want the fifth ability

Being able to run programs on shared, provable, authorized state is what lets people build applications
that no single company controls. A concrete motivating example in the Dash world is a stablecoin called
dash-dollar, a token meant to hold a steady value. Making one safely involves logic, minting tokens when
value comes in, redeeming them when it goes out, and doing the math correctly every time. Today that
math
has to run on servers that users must trust. If the platform could run the math itself, users would no
longer have to trust an operator to compute honestly, because every participant would recompute and
check it.

The goal is deliberately described as a general EXECUTION LAYER rather than as "smart contracts." A
smart contract is one popular shape for on-chain programs, associated with particular existing systems.
Saying "execution layer" keeps the question at the level of a general capability, the ability to run
programs at all, rather than smuggling in one specific style. dash-dollar is one motivating case, not
the
whole reason. The aim is a capability that helps the entire Dash toolset, with many possible uses.

## Part 2. How the work is done, and why the method is unusual

### Design on paper first, and why

I am not writing code. I am writing a design document and hardening it. For a system where a single
missed case can let someone steal funds or freeze the whole network, thinking is cheaper than building,
and a flaw caught on paper costs nothing, while the same flaw caught after launch can be catastrophic.

### The clean-room method

At the start I ran what is called a CLEAN-ROOM design exercise. Clean-room means each designer works in
isolation, without seeing the others' answers, so their agreements are meaningful rather than copied. I
wrote up the requirements with no hint of a preferred solution, then had several independent sources
each design the system from scratch, with no contact between them. Only after they had all committed
their independent answers were they compared.

Why several sources, and why genuinely independent ones. Sources that share an origin tend to share
blind spots, so their mistakes are correlated rather than independent. When two genuinely independent
sources reach the same conclusion, that agreement is strong evidence the conclusion is sound, the way
two witnesses who never met agreeing on a detail is stronger than one witness repeating themselves.
Agreement is weighted by source, not by headcount, because
two opinions from the same origin are really one opinion voiced twice.

Where the independent designs AGREED, that shared core is treated as forced by the requirements, the
answer any competent designer arrives at. Where they DIVERGED, each disagreement is treated as a real
decision to make deliberately, with reasons recorded.

### Adversarial review rounds, and "folding"

After the first design was synthesized, ADVERSARIAL REVIEW began. Adversarial means the reviewers are
told to attack, to hunt for the case that breaks the design, not to praise it. Each round, the current
design goes to the reviewer pool, their findings are collected, and the real ones are FOLDED back in,
meaning the design is revised to fix them and the version number bumped. I have done this nine times.
Version 1 came
from the clean-room round, and versions 2 through 9 each folded one review round.

Two habits keep this honest:

- A finding is never called "closed" just because a fix was written. A fix is a claim until an
  independent
  reviewer with access to the actual files confirms the flaw is gone. Confidence is graded in three
  levels. A bare
  assertion is the weakest, a claim checked against the written record is stronger, and a claim
  confirmed
  by actually running the system is strongest. Almost everything here is at the middle grade, because
  there is no running system yet.
- When one decision keeps drawing new findings round after round, the fix is to stop patching it piece
  by piece and rewrite it once, as a clean complete specification. Repeated patching tends to leave
  seams.
  This
  is exactly what happened with the hard problem described next.

### The reviewer pool, and a small experiment just closed

The standard pool is three independent sources. One of them runs with direct read access to the actual
project
files, which lets it catch contradictions between documents, not just reason about a pasted summary. The
other two receive a self-contained packet and reason from that. A second reviewer was recently given
full file access too, over two rounds, to see if it added value. It did not, in the sense that
matters, since it kept agreeing with the file-access reviewer it shares an origin with, and only ever
turned up
minor housekeeping issues, while the deepest new problems kept coming from the reviewers reasoning from
a
packet. So that extra full-access pass was retired. This is the kind of small process decision made and
recorded as the work goes.

## Part 3. The parts of the design that were settled early

Several big choices were agreed by the independent designs and have survived every review round. In
plain
terms:

- Programs run inside the record-keeping step itself, not off to the side. When the network processes a
  batch of changes, running the relevant program is part of processing that batch, so the program's
  effects are agreed by everyone at the same moment the rest of the record is.
- Programs are shipped in a portable, sandboxed format (the design uses WebAssembly, abbreviated WASM, a
  compact instruction format that many languages can compile to and that runs in a tightly controlled
  box). Sandboxed means the program cannot reach outside its box to touch the network, the clock, or the
  disk directly.
-  The only way a program can affect the outside world or the stored state is through a fixed menu of
  HOST
  FUNCTIONS. A host function is a specific, pre-approved operation the surrounding system offers, like
  "read this stored value" or "transfer this token." The program cannot invent new powers, it can only
  call items on the menu. This is the main safety boundary, the way an app on your phone can only do
  what
  the operating system's permissions allow.
- Everything a program stores goes into the same provable store as the rest of the platform, so proofs
  keep working for program data too.
- DETERMINISM is treated as the assumption whose failure would kill the design. Deterministic means the
  program produces the exact same output from the same input every single time, on every machine.
  Blockchains require this absolutely, because if two honest participants can compute different results,
  the shared record splits. A normal program is allowed to depend on the current time, on random
  numbers,
  or on the order threads happen to run in. On a blockchain none of that is allowed, because those
  things
  differ from machine to machine.
- RESOURCE ACCOUNTING is multidimensional. Every operation costs not one number but a small vector of
  costs across several dimensions, for example computation, storage, and memory, because a program can
  be
  cheap in one and expensive in another, and the network has a separate limited budget for each. The
  real platform already prices storage as a five-part vector, which happily matched this choice when it
  was measured.

## Part 4. The hard problem, in plain terms

The last several review rounds have all circled one issue, and it is worth understanding because it is
where blockchains are genuinely harder than ordinary software. The issue is what happens to
leftover
work and leftover obligations when something ENDS.

### Why "ending things" is hard here

In ordinary software, when you delete something, it is gone and you move on. On a blockchain, three
facts
make ending things hard at the same time:

-  Everyone must agree, and the work of ending something is real work (deleting records, updating
  indexes,
  paying out balances). That work costs the same limited per-block budget everything else does, so it
  cannot be unlimited or free, or an attacker could flood it and stall the network for everyone.
-  State is effectively permanent and must be paid for. You cannot silently drop data, because someone
  may
  hold a proof about it, and dropping it would either break that proof or, worse, let value be double
  counted. So "cleanup" is not free deletion, it is careful, paid, provable retirement.
- It must be BOUNDED. Bounded means there is a firm limit on how much of this ending-work can hit the
  network at any one moment, no matter how an attacker arranges things. Without a bound, a clever
  attacker
  could line up a huge pile of endings to all come due at once and jam the network.

### The vocabulary of "ending things"

A few terms recur:

- LEASE. Some stored state is rented rather than owned forever, so it expires unless renewed, like a
  storage-unit rental. When it expires, the unit has to be cleared out, which is real cleanup work.
- CUSTODY. When a program holds assets on a user's behalf, that holding is called a custody position,
  like a coat check holding your coat. If the program is shut down, the coats still have to get back to
  their owners.
- OBLIGATION. A promise the system has taken on that will require work later, for example "when this
  request finishes, pay out this amount" or "when this record is removed, clean up its index entries."
- RETIREMENT. The controlled shutdown of a program version or of a whole execution environment (a shared
  space several programs run in). Retirement is not a crash. It is a deliberate, orderly ending, and the
  design has to say exactly what happens to every kind of leftover when it occurs.
- TERMINAL WORK. The umbrella term for all the work that ending something creates: paying out custody,
  settling in-flight requests, tombstoning (marking-as-dead) scheduled tasks, and physically reclaiming
  storage. "Terminal" here just means end-of-life, as in a terminal stage, not a bus terminal.

The recurring hard question has been how to guarantee that all this terminal work is paid for, is
bounded so it cannot flood the network, and still lets legitimate users end things whenever they need
to,
including all at once.

## Part 5. The current mechanism (version 9), each piece with an analogy

Version 9 pulls the previous piecemeal answers into one coherent mechanism. Here is each piece in plain
terms. The formal version is decision D3b in `DESIGN.md`, with related decisions D6, D15, and D16.

### The block budget, and why bounding matters (the kitchen with limited burners)

Every block (each batch of changes the network processes together) has a fixed capacity, like a kitchen
with a fixed number of burners. Ordinary user transactions, and all this ending-work, compete for those
burners. The design reserves a guaranteed slice of the burners for cleanup so it always makes progress,
while making sure the reserved slices plus everything else can never add up to more burners than the
kitchen has. If the arithmetic of these reserved slices is wrong, either cleanup starves or the block is
over-booked, and much of the design work has been getting that arithmetic exactly right and provably so.

### The terminal-work meter (a move-out deposit that tracks your stuff)

The central version-9 idea is a TERMINAL-WORK METER. Every object that will someday need ending-work
carries an honest, up-to-date estimate of the worst-case work its ending will require, measured in the
same multidimensional units as everything else. Think of a rental deposit that is not a flat fee but is
sized to how much stuff you actually keep in the unit, and is topped up whenever you move more in. The
key
property, which earlier versions got wrong, is that this estimate is a LIVE INVARIANT: it is set when
the
object is created and updated on every change that makes the object bigger. An earlier version funded
the
move-out cost once at move-in and then let tenants pile in more furniture for free, so the eventual
cleanup cost outran what had been set aside. Version 9 charges the extra at the moment you bring in more
furniture.

Crucially, this deposit is measured in ENDING-WORK units, not in storage bytes. An earlier version tried
to reuse the storage-rent meter as if it also measured ending-work, but some things generate a lot of
ending-work while taking almost no permanent storage, so the storage meter missed them. Version 9 keeps
the two meters separate, one for how much permanent space you occupy, one for how much work your
eventual
departure will cost.

### Deadline work versus no-deadline work (two different queues)

Some ending-work has a hard deadline and some does not, and version 9 stops treating them the same.

- Some in-flight requests must finish by a specific time or they are declared failed. That is
  deadline-bearing work. For these, the system sets aside a guaranteed slot at the deadline in advance,
  like reserving an operating-room slot for a scheduled surgery, so the work is guaranteed to happen on
  time and can never be crowded out.
- Storage cleanup usually has no hard deadline. Nobody is harmed if a dead record is physically erased a
  bit later, as long as it is already treated as dead. This deadline-free work goes into a steady
  cleanup
  queue that drains at a fixed pace.

An earlier version put the deadline-bearing work into the slow no-deadline queue, so a big pile of
cleanup
could push a time-sensitive request past its deadline and turn a mere delay into an actual loss. That
was
a genuine flaw, and separating the two queues fixes it.

### Rate-matching (do not admit guests faster than the cleaning crew can turn over rooms)

For the no-deadline cleanup queue, version 9 uses a RATE-MATCHING rule. The queue drains at a fixed
rate,
so many rooms per hour. As long as new guests who will eventually need their rooms cleaned check in no
faster than the crew turns rooms over, the backlog stays under control, even though the hotel can hold
any
number of guests over time. The safety point is subtle and important. When a whole environment shuts
down
at once, the system does NOT try to clean every room in that instant. It immediately marks all those
rooms
vacant (which is cheap and instant, so nothing is unsafe), then cleans them at the steady rate over
however many blocks it takes. This means the total number of live objects is NOT capped, which matters
because an earlier version accidentally capped it and would have throttled the whole platform's growth.

### Single-owner versus shared custody (one claim ticket, or a shared locker)

When a program holds assets and then shuts down, who gets the assets out. Version 9 splits custody by
who
can claim it:

- SINGLE-OWNER custody is a coat check with exactly one claim ticket held by one person. That person can
  walk up and pull their coat out directly, through a built-in native path that does not need the
  shut-down
  program to run. Because a single motivated owner will come get their own property, the system does not
  have to schedule and guarantee that payout itself, which keeps things simple and uncapped.
-  AUTONOMOUS or SHARED custody is a locker several parties share, or one controlled by program logic
  with
  no single human key-holder. Nobody can individually walk up and claim the whole thing. For these, the
  system itself must schedule the payout, so they stay inside the guaranteed, budgeted queue.

An earlier version assumed every custody position had a single owner who could come claim it, which
stranded the shared ones, they would be frozen safely but unreachable forever. Version 9 handles the
shared case by keeping it in the managed queue.

### Get your coat now, tidy the closet later (logical exit versus physical cleanup)

When the single owner claims their coat, version 9 makes that a two-step affair. Step one, the owner
immediately gets their coat and their claim is settled, a small fixed-size action that always fits in
the
budget. Step two, the now-empty hook and the paperwork are tidied up later through the steady cleanup
queue. An earlier version tried to do both at once, in a single action, which could exceed the
per-action
size limit for a customer who had somehow accumulated an enormous amount of paperwork, and then the
claim
would get stuck and the owner locked out. Splitting it guarantees the owner is never locked out.

### Re-checking the claim ticket if it changes hands (dynamic classification)

The single-owner path only works if there really is a single owner. But ownership can change after the
fact, the claim ticket can be split among several people, or handed to a committee, or transferred into
a
shared locker. If that happens and nobody notices, you are back to the stranded-shared-locker problem.
Version 9 fixes this by re-checking the classification every time the claim authority changes. If a
change
would turn a single-owner position into a shared one, the system either keeps a valid single-owner path
or
moves the position into the managed queue then and there, funding its future cleanup at that moment, and
refuses the change if that funding cannot be arranged. It deliberately does nothing about a person
simply
losing their only key, because the system cannot detect that, and it remains the owner's own risk, the
way
a bank cannot help you if you lose the only key to your safe-deposit box and tell no one.

## Part 6. How the story ended, and what "finished" means here

This section replaces the mid-flight snapshot that stood here at version 9. The review ran twelve rounds
in all, and on 2026-08-10 it ENDED: round 12 came back with a clean approval, no findings, from every
reviewing family, including the reviewer that reads the actual project files. By the rule set at the
start (the review only ends when a full fresh attack finds nothing real to fix), that was the finish
line, and the design is now called REVIEW-COMPLETE at version 12.

### The endgame, in plain terms

After version 9 introduced the terminal-work meter (the move-out deposit from Part 5), the last three
rounds were about progressively smaller cracks at its edges, and each round's findings fit inside the
previous round's fix:

- Round 10 caught that when a custody position is TRANSFERRED to a different program (think of moving
  your account to a new bank), the old account's cleanup deposit was being handed back too early,
  before the old paperwork was actually shredded. The fix rides the deposit along with the shredding
  job itself, releasing it only when the shredding is done, and makes every transfer pay full price for
  the new account rather than recycling the old payment. Round 10 also caught that the rules for
  changing an account's ownership type only covered one direction (single owner becoming a group) and
  not the reverse, and that one sentence promised more fairness than the mechanism delivers.
- Round 11 certified all of those fixes and left exactly one complaint, from one reviewer, that a
  definition was AMBIGUOUS: the deposit is supposed to track not just how much ending-work an account
  will need but WHICH cleanup crew does it, and the text only implied that through an example. The
  other reviewers read it the intended way, but a rule that has to be read charitably is not finished.
- Round 12 reviewed the two sentences that made the definition explicit, and every reviewer, including
  the one who had complained, approved with nothing further.

That shrinking pattern (eight findings, then four, then three, then one, then a wording issue, then
none) is what convergence looks like from the inside, and it is why the clean pass at the end means
something. It was earned against reviewers who had found real problems in every earlier version.

### What "review-complete" honestly means

It means three independent review sources, one of them reading the real files, attacked the full
design and found nothing left to fix in the TEXT. It is a strong statement about the design on paper
and deliberately nothing more. Nothing has been built, so nothing has been tested by running it, and
by this project's own evidence rules no part of the design is "closed" in the strong sense until an
implementation exists and independent review confirms the behaviour against it. Think of it as a
building's blueprints passing every independent engineering review commissioned on them, a real
milestone, and still not a building.

### What remains, concretely

- THE NUMBERS. The design's mechanisms are written in terms of dials nobody has set: how fast the
  cleanup crew works, how big the deposits are, how much capacity the scheduled-settlement calendar
  reserves. A companion document (`docs/PHASE0_VERIFY_ESTIMATES.md`) sorts every remaining number by
  what would actually produce it, derives rough bounds where the earlier measurements allow (for
  example, a minimal custody record's deposit works out to roughly the cost of creating one ordinary
  document, and an attacker trying to flood permanent storage pays a steep, calculable price per
  block), and concludes that most of the numbers need one buildable artifact, a METERING PROTOTYPE, a
  small test harness that prices the ending-work operations against the real storage engine, rather
  than the whole virtual machine.
- ONE GOVERNANCE CHOICE, deliberately left to a human decision rather than forced by the design: what
  ultimately happens to an abandoned custody balance whose owner never returns.
- POSITIONING. The research summary (`SUMMARY.md`) presents the whole record as input to a general
  execution-layer effort across use cases, with recommendations, including the standing advice that
  the dash-dollar stablecoin should ship on its current design and treat on-chain math as a later
  upgrade rather than a launch dependency.

And one standing rule protects the result. If anyone substantively changes the design record in the
future, it loses the review-complete label until a fresh adversarial round has attacked the change.

## Part 7. Turning the design's numbers from guesses into measurements

When the review closed, the design was sound on paper but still written in terms of numbers nobody had
measured. How fast does the cleanup crew work? How big is a record's move-out deposit? How much of each
block should the calendar of scheduled work reserve? The design named these dials without setting them,
because setting them accurately needs measurement, not argument.

So the next step was to build a small prototype. Not the whole execution layer, which is a large
project, but a focused test harness that measures the dials against the REAL storage engine the platform
uses (the authenticated store from Part 1). The work ran in five stages, and each one produced numbers
rather than claims.

- STAGE A calibrated the ruler. Before measuring anything you have to trust your measuring tool, so this
  stage ran real storage operations and compared their actual cost against the estimate the storage
  engine ships with. The finding is that for durable storage, the engine's own worst-case estimate is
  exact,
  and it does not drift as the database grows. For one other cost (the ripple of updating parent
  records up the tree) the built-in worst-case estimate is far too pessimistic, so that cost should be
  measured directly rather than taken from the worst case. Knowing which estimates to trust is what
  is what keeps the later stages sound.
- STAGE B proved the meter's rules in a runnable model. The move-out deposit and the cleanup queue from
  Part 5 were written as a small program, and the ten safety guarantees the reviewers cared about were
  turned into automatic checks. The important discipline here is that each guarantee was first
  DELIBERATELY BROKEN, to confirm the check actually catches the break, before confirming it passes when
  the code is correct. That paid off immediately, because the first attempt to break one rule did not
  actually
  break anything, which revealed that the test was aimed at the wrong hazard. It was corrected, and only
  then did all ten checks pass. A test never watched failing is not yet a test.
- STAGE C measured what each kind of record really costs to clean up. Every kind of held position (a
  single owner's balance, an automatic payout, a scheduled task, and so on) was built in the REAL
  storage engine, using stand-in record shapes rather than Platform's own document types, and then fully
  wound down, measuring the cost at each step. Two results stood out.
  First, what a record uses when it is created is exactly what its cleanup gives back, to the byte, for
  every kind. That matters because the whole move-out-deposit idea assumes the ending cost is a real,
  recoverable quantity, and the measurement says it is. Second, an automatic payout to many recipients
  costs in exact proportion to how many, while a single owner's cash-out adds no durable cost at all
  when it ends, which confirms the late decision that a single owner's exit is paid by that owner and
  does not burden the shared cleanup lane.
- STAGE D ran the whole thing under heavy load. Feeding cleanup work in faster than the crew can handle
  it, the safety valve (the flow ceiling from Part 5) held the backlog at zero by throttling intake,
  while a version with the valve removed piled up without bound. That contrast is the valve's value,
  measured rather than asserted. This stage also turned a chosen crew speed into a real throughput
  figure, and it surfaced a genuine DEFECT, though it was not read as one at the time. One automatic payout
  fanning out to sixty-four recipients costs more than an entire block's cleanup budget. This was
  originally written up here as a constraint the design anticipates, on the grounds that the work would
  be spread across several blocks. CORRECTED 2026-08-30. The model has no way to do that. The queue
  takes one whole item at a time and stops at the first item too big to fit, so an item larger than a
  block's budget is never processed and blocks everything queued behind it. Spreading the work is what
  the design intends and what the runnable model does not implement, which is now an open defect with
  the repair still to be chosen. A
  test that retired a hundred thousand positions at once drained them in exactly the predicted number of
  blocks, with every safety guarantee holding at every step.
- STAGE E measured the one thing the earlier stages could not, the cost of the actual COMPUTATION a
  program does. The difficulty is that a blockchain cannot measure this in seconds, because every
  computer must agree on the exact figure, and clock speeds differ. The solution is to count operations
  instead, using a measure called FUEL that a real program interpreter tracks deterministically. A fixed
  arithmetic workload came out to a steady sixteen fuel per computation step, identical on every run.
  That gives computation the same kind of countable, agreed-upon unit that storage bytes already have.

After those five stages, all four kinds of cost the meter needs (durable storage, the update ripple,
cleanup-crew throughput, and computation) have measured units. What is left is not measurement. One
remaining number depends on a guess about the real mix of applications people will run, which is a
modeling choice, and another depends on a survey of the actual hardware masternode operators use, which
no amount of code can produce. The measurable dials are measured.

The same caution from Part 6 applies to all of this. These are measurements of the cost model against
the real storage engine, not of a finished platform running the execution layer, because that platform
does not exist yet. They turn the design's open numbers from guesses into measured quantities, and no
more than that.

## Part 8. Adopting a proven runner instead of building one (the CosmWasm question)

Everything up to here assumed the program-runner would be BUILT, designing the sandbox, the
metering, and all the rest from scratch. Partway through, a Dash Platform core developer raised a
different route, namely to ADOPT an existing, proven runner and connect it to Platform's own storage
rather than build a new one. The runner in question is called CosmWasm.

In plain terms, CosmWasm is a mature, widely-used engine for running exactly the kind of sandboxed
WebAssembly programs the design already settled on (Part 3). A whole family of blockchains runs on it,
with years of production use behind it. Adopting it would be less like designing a new car engine and
more like dropping in a proven engine that already fits the chassis, if it really does fit.

The catch is the same one that ruled out Ethereum. A runner is usable here only if a program's data can
live in Platform's authenticated store and stay provable to light clients (the tamper-evident receipts
from Part 1). The gating question was whether CosmWasm fails that test the way Ethereum does.

It does not, and the reason is structural rather than lucky. CosmWasm was designed from the start to sit
on an authenticated, ordered filing system with the very shape Platform's store already has (ordered
keys, range lookups, cryptographic proofs), because the blockchains that use it back it with a store of
that same kind. Putting CosmWasm on Platform's store is therefore a swap between two stores of the same
type, not a graft of a foreign one. That is exactly the affinity the EVM lacks.

To move this from argument to evidence, I built small running spikes, and they worked at every layer.

- CosmWasm's storage ran over Platform's real store, with reads, writes, ordered lookups inside a
  transaction, and cryptographic proofs of the result.
- A real, already-existing compiled program ran start to finish through the actual CosmWasm engine over
  that store, and the data it wrote was provable.
- The program reached Platform's own tokens through the engine, reading a balance and applying a
  transfer to real balances in the store, with the outcome proven.
- Its running cost was priced by the store's own measured cost rather than a flat guess, which is if
  anything an improvement over how the blockchains that use CosmWasm price it.

Where the next part (Ethereum) was first written as "reachable in principle," this part is "shown by
running code."

CORRECTED 2026-08-30. This section previously recommended adopting CosmWasm and described the remaining
work as bounded engineering. Both overstate where the research stands.

CosmWasm is the LEADING CANDIDATE, backed by Platform's store, with Ethereum compatibility treated as a
separate later layer. Two things support that. The hardest question, provability, is answered with
running evidence. And the fit is structural rather than coincidental.

What remains is NOT merely bounded wiring. Two defects have since been found in the prototype itself, a
storage scan that does unbounded work before it is charged for it, and a cleanup item that can grow too
large to ever be processed and then blocks everything queued behind it. The evidence also sits on a
software line whose security support ended in April 2025. A second engine, MoveVM, has since cleared the
same screen and has no evidence against it either way. None of the five conditions has been met and two
have failed outright. CORRECTED 2026-08-30. This previously said building a bespoke runner would be justified only if some
need proved unmeetable within CosmWasm. That put the burden of proof on every alternative and let the
leader win by default. CosmWasm leads on VOLUME OF EVIDENCE, nothing more. Which engine is best, and
whether to adopt one at all rather than build, are both unresolved, and the second engine that clears
the same screen has had no work done against it in either direction.

What this is NOT, stated plainly. The spikes are small demonstrations, not a finished integration, and
connecting CosmWasm to the production node and binding every Dash-native feature is real work. Before
committing, five questions need verified answers rather than assurances, each one a place where a wrong
call would show up as a split record or a lost platform property:

- that the engine computes identically on every machine (the determinism rule from Part 3, the failure
  that presents as a split record);
- that a program's work plus its storage writes plus its proofs still fit inside a block's time budget,
  even under deliberate attack;
- that a program can check a zero-knowledge privacy proof on-chain, which is a stated goal of the
  project;
- that slow native operations such as the masternode group-signature are handled as "start now, finish
  in a later block" rather than as an instant call;
- that the exact engine version is pinned identically across every machine, since a version mismatch in
  a deterministic engine is itself a way for the record to split.

The full form of this decision, its evidence, and the five gates are in
`docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md`.

## Part 9. The Ethereum question, and what "substrate" means

Anyone who has followed smart contracts will ask whether this makes Dash compatible with Ethereum, the
best-known platform for on-chain programs. The accurate answer has an order to it, and the order is
worth getting right, because it is easy to state as either more or less than it is.

Platform is NOT Ethereum-compatible at its foundation, and that was a deliberate, unanimous decision.
Ethereum's virtual machine, the EVM (Ethereum Virtual Machine), organizes each program's data in a
fundamentally different way from Dash Platform, using its own kind of storage tree and its own hashing
throughout. Every one of the independent designs rejected building on the EVM directly, for the same
reason, which is that doing so would break the light-client provability that makes Platform special (the
tamper-evident
receipts from Part 1), or it would force an awkward wrapper around every native feature Platform already
has. On this the designs agreed completely.

That is not the end of the story, though, because rejecting the EVM as the FOUNDATION is different from
ruling out Ethereum compatibility as a later, optional LAYER. This is where the word SUBSTRATE comes in.
A substrate is a base that other things can be built or run on top of. The design that was chosen, a
sandboxed program-runner metered in the platform's own credits, turns out to be a good substrate for
running an Ethereum interpreter as just another deployed program. A useful comparison is running a
Windows program on a Mac through a compatibility layer. The Mac is not Windows, and it never becomes
Windows, but it can run a Windows program inside a controlled box. In the same way, Platform would not
become Ethereum, but it could host an Ethereum program inside its own sandbox, with that program's data
kept in Platform's storage and its work paid for in Platform's credits.

Several things make that route genuinely reachable rather than wishful. Ethereum programs are already
the
kind of exact, integer-only computation Platform requires, so they fit the determinism rule without
change. Dash already uses the same signature mathematics Ethereum does, so the one piece every Ethereum
program leans on is native. And because the Ethereum interpreter would be just another program, adding
it
would not touch the network's core agreement rules at all.

This is no longer only an argument. I built a small running spike of exactly this shape. A minimal
Ethereum interpreter was written as an ordinary program on the runner from Part 8 (a CosmWasm program),
and it was run: it executed a short piece of real Ethereum program-code, the value that code stored
landed in Platform's store through the storage path Part 8 demonstrated, and that stored value was
provable in Platform's own receipt format. It is a handful of Ethereum's operations, not the whole set,
and a tiny program, so it demonstrates the one load-bearing point (an Ethereum guest's data can live in
Platform's store and be proven) and nothing about being complete or fast.

Some things would not come for free, and a community weighing this should hear them plainly. Ethereum
wallets and block explorers expect proofs and data in Ethereum's exact format, so they would not work
directly, and a translation layer would sit between them and Platform. Ethereum's particular hashing and
its own internal pricing are not native costs, so they would run somewhat more expensively than
Platform's own operations. And the cleanup accounting from Part 5 would need a new category to handle
Ethereum-style storage, which never expected to be leased or reclaimed. None of these is a wall, but
each
is real work.

There is also a lighter option worth knowing about. Instead of running Ethereum's programs as-is, one
could let people write in Ethereum's popular programming language and compile it to run on Platform's
own
runner. That gives developers a familiar language without promising that existing Ethereum programs run
unchanged. It is familiarity, not drop-in compatibility, and it is much less work.

The recommendation, and it is a recommendation rather than a plan, is that Ethereum compatibility
should be treated as its own separate design project if it is ever pursued. The one-tiny-program proof
this part used to name as the prerequisite has now been built (the spike above), so the feasibility of
the guest shape is settled, and what remains is the hard parts, a full Ethereum engine and the
translation layer for Ethereum's wallets and tools. Those should be proven out in that separate project
rather than bolted onto the base design as an afterthought.

## Glossary

- Adversarial review: a review whose reviewers are told to attack the design and find what breaks it.
- Authenticated store: a data store that can produce cryptographic proofs about what it contains.
- Autonomous custody: assets held with no single external owner who can claim them, so the system must
  release them itself.
- Block: one batch of changes the network processes and agrees on together.
- Clean-room design: independent designers working in isolation so their agreements are meaningful.
- CosmWasm: a mature, widely-used engine for running sandboxed WebAssembly programs, used across a
  family of blockchains. This project's spikes demonstrate it running over Platform's authenticated
  store with provability preserved. CORRECTED 2026-08-30, this previously said that is why the project
  recommends adopting it. It is the LEADING CANDIDATE under evaluation rather than a choice, and it may
  lose. No gate has passed and two fail.
- Cryptographic proof: evidence that relies on hard mathematics rather than on trusting the sender.
- Custody position: assets a program holds on someone's behalf.
- Data contract / document: the platform's names for a database schema and a database row.
- Deterministic: producing the identical result from the same input on every machine, every time.
- EVM (Ethereum Virtual Machine): the program-running engine that Ethereum uses, with its own way of
  organizing storage and its own internal pricing.
- Execution layer: the general ability to run programs on the shared state, the thing this project
  explores adding.
- Source (of a design or a review): one independent origin. Two opinions from the same origin tend to
  share blind spots, so they count as one voice rather than two.
- Flow ceiling: a limit that keeps new cleanup work from entering faster than the cleanup crew can
  clear it, which is what keeps the backlog bounded.
- Folding: revising the design to incorporate a review round's confirmed findings, and bumping the
  version.
- Fuel: a deterministic count of the operations a program runs, used to price computation in a way every
  computer agrees on, since clock time would not.
- Host function: a specific pre-approved operation the surrounding system offers to a program.
- Lease: stored state that is rented and expires unless renewed.
- Metering / resource accounting: measuring and charging what each operation costs, here as a
  multidimensional vector.
- Obligation: a promise the system has taken on that will require work later.
- Rate-matching: keeping the inflow of future cleanup no faster than the cleanup queue can drain.
- Retirement: the deliberate, orderly shutdown of a program version or an execution environment.
- Sandbox: a tightly controlled box a program runs in, unable to reach outside except through host
  functions.
- Substrate: a base that other things can be built or run on top of. The chosen design could act as a
  substrate that hosts an Ethereum program later, without the platform itself becoming Ethereum.
- Terminal work: all the end-of-life work that ending something creates.
- WebAssembly (WASM): a compact, portable, sandboxable instruction format many languages compile to.
