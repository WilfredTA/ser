# Ser

Symbolic EVM in Rust


## Introduction
Ser's design is informed by lessons I learned from a previous attempt to build a highly abstract & generalized symbolic execution framework parameterized over instruction sets (we called it symbolic-stack-machines). Unlike symbolic-stack-machines, Ser only supports a single architecture: the EVM.
 

Ser is designed with two concepts in mind:
1. An instruction is the most granular state transition, and thus instructions are functions over machine states.
2. It is easier to analyze program behavior when the state transitions are recoverable, reversible, and manipulatable.

Due to item 1, each instruction has an `exec` method which takes as an argument a complete machine. Due to item 2, `exec` does not produce side effects nor does it produce a new machine. Rather, it produces a lightweight *description* of *how* the instruction would change the machine.
