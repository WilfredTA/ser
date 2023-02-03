# Ser

Symbolic EVM in Rust


## Introduction
Ser's design is informed by lessons I learned from a previous attempt to build a highly abstract & generalized symbolic execution framework parameterized over instruction sets (we called it [symbolic-stack-machines](https://github.com/WilfredTA/symbolic-stack-machines)). Unlike symbolic-stack-machines, Ser only supports a single architecture: the EVM.
 

Ser is designed with two concepts in mind:
1. An instruction is the most granular state transition, and thus instructions are functions over machine states.
2. It is easier to analyze program behavior when the state transitions are recoverable, reversible, and manipulatable.

Due to item 1, each instruction has an `exec` method which takes as an argument a complete machine. Due to item 2, `exec` does not produce side effects nor does it produce a new machine. Rather, it produces a lightweight *description* of *how* the instruction would change the machine.



## Design

Symbolic EVMs differ in the following ways:
1. Which aspects of the EVM are modeled symbolically versus concretely
2. How frequently SMT solving is invoked 
3. How analyses are performed & intended to be used

Ser models the stack and storage fully symbolically, and models memory partially symbolically (memory values are symbolic, but indexes must be concrete).

Ser currently (and inefficiently) builds out a complete tree of all possible traces. Only after this tree is constructed does it begin utilizing SMT solving to check which end states are reachable or not.

Ser is intended to be used primarily as a *library* (as opposed to a standalone tool or CLI application) or *backend* for smart contract testing tools.

Ser also differs from most symbolic EVM tools in that state transitions in the EVM are implemented as independent objects which can later be committed, rewound, or logged in a structured way. This has multiple practical benefits for other tools that may use Ser behind the scenes. 

The tradeoff is that this approach - as it is currently implemented - uses more memory than if the execution of each instruction and its effect on the machine state were coupled together.