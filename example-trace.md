

```
------- Reachable State ---------
State EvmState {
    memory: Memory {
        inner: [],
    },
    stack: Stack {
        stack: [
            BitVec {
                inner: Z3(
                    #x0000000000000000000000000000000000000000000000000000000000000002,
                ),
                typ: Z3,
            },
            BitVec {
                inner: Z3(
                    #x0000000000000000000000000000000000000000000000000000000000000064,
                ),
                typ: Z3,
            },
            BitVec {
                inner: Z3(
                    #x0000000000000000000000000000000000000000000000000000000000000032,
                ),
                typ: Z3,
            },
        ],
    },
    pc: 8,
    pgm: [
        Push32(
            BitVec {
                inner: Z3(
                    #x0000000000000000000000000000000000000000000000000000000000000002,
                ),
                typ: Z3,
            },
        ),
        Push32(
            BitVec {
                inner: Z3(
                    #x0000000000000000000000000000000000000000000000000000000000000001,
                ),
                typ: Z3,
            },
        ),
        Push32(
            BitVec {
                inner: Z3(
                    a,
                ),
                typ: Z3,
            },
        ),
        Add,
        Push32(
            BitVec {
                inner: Z3(
                    #x0000000000000000000000000000000000000000000000000000000000000007,
                ),
                typ: Z3,
            },
        ),
        JumpI,
        Push(
            BitVec {
                inner: Z3(
                    #x0000000000000000000000000000000000000000000000000000000000000064,
                ),
                typ: Z3,
            },
        ),
        Push32(
            BitVec {
                inner: Z3(
                    #x0000000000000000000000000000000000000000000000000000000000000032,
                ),
                typ: Z3,
            },
        ),
    ],
}


MODEL:Some(
    a -> #xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    ,
)



------- Full Trace Tree -------
StateTree {
    id: NodeId {
        id: 763425973fbb4e2fba219c2b0b9e051a,
        parent: None,
    },
    val: EvmState {
        memory: Memory {
            inner: [],
        },
        stack: Stack {
            stack: [],
        },
        pc: 0,
        pgm: [
            Push32(
                BitVec {
                    inner: Z3(
                        #x0000000000000000000000000000000000000000000000000000000000000002,
                    ),
                    typ: Z3,
                },
            ),
            Push32(
                BitVec {
                    inner: Z3(
                        #x0000000000000000000000000000000000000000000000000000000000000001,
                    ),
                    typ: Z3,
                },
            ),
            Push32(
                BitVec {
                    inner: Z3(
                        a,
                    ),
                    typ: Z3,
                },
            ),
            Add,
            Push32(
                BitVec {
                    inner: Z3(
                        #x0000000000000000000000000000000000000000000000000000000000000007,
                    ),
                    typ: Z3,
                },
            ),
            JumpI,
            Push(
                BitVec {
                    inner: Z3(
                        #x0000000000000000000000000000000000000000000000000000000000000064,
                    ),
                    typ: Z3,
                },
            ),
            Push32(
                BitVec {
                    inner: Z3(
                        #x0000000000000000000000000000000000000000000000000000000000000032,
                    ),
                    typ: Z3,
                },
            ),
        ],
    },
    path_condition: None,
    left: Some(
        StateTree {
            id: NodeId {
                id: 93e5d25a069f4945afbc7202ca712223,
                parent: None,
            },
            val: EvmState {
                memory: Memory {
                    inner: [],
                },
                stack: Stack {
                    stack: [
                        BitVec {
                            inner: Z3(
                                #x0000000000000000000000000000000000000000000000000000000000000002,
                            ),
                            typ: Z3,
                        },
                    ],
                },
                pc: 1,
                pgm: [
                    Push32(
                        BitVec {
                            inner: Z3(
                                #x0000000000000000000000000000000000000000000000000000000000000002,
                            ),
                            typ: Z3,
                        },
                    ),
                    Push32(
                        BitVec {
                            inner: Z3(
                                #x0000000000000000000000000000000000000000000000000000000000000001,
                            ),
                            typ: Z3,
                        },
                    ),
                    Push32(
                        BitVec {
                            inner: Z3(
                                a,
                            ),
                            typ: Z3,
                        },
                    ),
                    Add,
                    Push32(
                        BitVec {
                            inner: Z3(
                                #x0000000000000000000000000000000000000000000000000000000000000007,
                            ),
                            typ: Z3,
                        },
                    ),
                    JumpI,
                    Push(
                        BitVec {
                            inner: Z3(
                                #x0000000000000000000000000000000000000000000000000000000000000064,
                            ),
                            typ: Z3,
                        },
                    ),
                    Push32(
                        BitVec {
                            inner: Z3(
                                #x0000000000000000000000000000000000000000000000000000000000000032,
                            ),
                            typ: Z3,
                        },
                    ),
                ],
            },
            path_condition: None,
            left: Some(
                StateTree {
                    id: NodeId {
                        id: c31b9dd6c241473ebaecb540353f4925,
                        parent: None,
                    },
                    val: EvmState {
                        memory: Memory {
                            inner: [],
                        },
                        stack: Stack {
                            stack: [
                                BitVec {
                                    inner: Z3(
                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                    ),
                                    typ: Z3,
                                },
                                BitVec {
                                    inner: Z3(
                                        #x0000000000000000000000000000000000000000000000000000000000000001,
                                    ),
                                    typ: Z3,
                                },
                            ],
                        },
                        pc: 2,
                        pgm: [
                            Push32(
                                BitVec {
                                    inner: Z3(
                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                    ),
                                    typ: Z3,
                                },
                            ),
                            Push32(
                                BitVec {
                                    inner: Z3(
                                        #x0000000000000000000000000000000000000000000000000000000000000001,
                                    ),
                                    typ: Z3,
                                },
                            ),
                            Push32(
                                BitVec {
                                    inner: Z3(
                                        a,
                                    ),
                                    typ: Z3,
                                },
                            ),
                            Add,
                            Push32(
                                BitVec {
                                    inner: Z3(
                                        #x0000000000000000000000000000000000000000000000000000000000000007,
                                    ),
                                    typ: Z3,
                                },
                            ),
                            JumpI,
                            Push(
                                BitVec {
                                    inner: Z3(
                                        #x0000000000000000000000000000000000000000000000000000000000000064,
                                    ),
                                    typ: Z3,
                                },
                            ),
                            Push32(
                                BitVec {
                                    inner: Z3(
                                        #x0000000000000000000000000000000000000000000000000000000000000032,
                                    ),
                                    typ: Z3,
                                },
                            ),
                        ],
                    },
                    path_condition: None,
                    left: Some(
                        StateTree {
                            id: NodeId {
                                id: 9b647bf4ac7b46a69242e59984017fef,
                                parent: None,
                            },
                            val: EvmState {
                                memory: Memory {
                                    inner: [],
                                },
                                stack: Stack {
                                    stack: [
                                        BitVec {
                                            inner: Z3(
                                                #x0000000000000000000000000000000000000000000000000000000000000002,
                                            ),
                                            typ: Z3,
                                        },
                                        BitVec {
                                            inner: Z3(
                                                #x0000000000000000000000000000000000000000000000000000000000000001,
                                            ),
                                            typ: Z3,
                                        },
                                        BitVec {
                                            inner: Z3(
                                                a,
                                            ),
                                            typ: Z3,
                                        },
                                    ],
                                },
                                pc: 3,
                                pgm: [
                                    Push32(
                                        BitVec {
                                            inner: Z3(
                                                #x0000000000000000000000000000000000000000000000000000000000000002,
                                            ),
                                            typ: Z3,
                                        },
                                    ),
                                    Push32(
                                        BitVec {
                                            inner: Z3(
                                                #x0000000000000000000000000000000000000000000000000000000000000001,
                                            ),
                                            typ: Z3,
                                        },
                                    ),
                                    Push32(
                                        BitVec {
                                            inner: Z3(
                                                a,
                                            ),
                                            typ: Z3,
                                        },
                                    ),
                                    Add,
                                    Push32(
                                        BitVec {
                                            inner: Z3(
                                                #x0000000000000000000000000000000000000000000000000000000000000007,
                                            ),
                                            typ: Z3,
                                        },
                                    ),
                                    JumpI,
                                    Push(
                                        BitVec {
                                            inner: Z3(
                                                #x0000000000000000000000000000000000000000000000000000000000000064,
                                            ),
                                            typ: Z3,
                                        },
                                    ),
                                    Push32(
                                        BitVec {
                                            inner: Z3(
                                                #x0000000000000000000000000000000000000000000000000000000000000032,
                                            ),
                                            typ: Z3,
                                        },
                                    ),
                                ],
                            },
                            path_condition: None,
                            left: Some(
                                StateTree {
                                    id: NodeId {
                                        id: 66c4de3ffa364582a08255e69c277215,
                                        parent: None,
                                    },
                                    val: EvmState {
                                        memory: Memory {
                                            inner: [],
                                        },
                                        stack: Stack {
                                            stack: [
                                                BitVec {
                                                    inner: Z3(
                                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                                    ),
                                                    typ: Z3,
                                                },
                                                BitVec {
                                                    inner: Z3(
                                                        (bvadd a #x0000000000000000000000000000000000000000000000000000000000000001),
                                                    ),
                                                    typ: Z3,
                                                },
                                            ],
                                        },
                                        pc: 4,
                                        pgm: [
                                            Push32(
                                                BitVec {
                                                    inner: Z3(
                                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                                    ),
                                                    typ: Z3,
                                                },
                                            ),
                                            Push32(
                                                BitVec {
                                                    inner: Z3(
                                                        #x0000000000000000000000000000000000000000000000000000000000000001,
                                                    ),
                                                    typ: Z3,
                                                },
                                            ),
                                            Push32(
                                                BitVec {
                                                    inner: Z3(
                                                        a,
                                                    ),
                                                    typ: Z3,
                                                },
                                            ),
                                            Add,
                                            Push32(
                                                BitVec {
                                                    inner: Z3(
                                                        #x0000000000000000000000000000000000000000000000000000000000000007,
                                                    ),
                                                    typ: Z3,
                                                },
                                            ),
                                            JumpI,
                                            Push(
                                                BitVec {
                                                    inner: Z3(
                                                        #x0000000000000000000000000000000000000000000000000000000000000064,
                                                    ),
                                                    typ: Z3,
                                                },
                                            ),
                                            Push32(
                                                BitVec {
                                                    inner: Z3(
                                                        #x0000000000000000000000000000000000000000000000000000000000000032,
                                                    ),
                                                    typ: Z3,
                                                },
                                            ),
                                        ],
                                    },
                                    path_condition: None,
                                    left: Some(
                                        StateTree {
                                            id: NodeId {
                                                id: 850f56c7fb474be8964df31b0768e4e0,
                                                parent: None,
                                            },
                                            val: EvmState {
                                                memory: Memory {
                                                    inner: [],
                                                },
                                                stack: Stack {
                                                    stack: [
                                                        BitVec {
                                                            inner: Z3(
                                                                #x0000000000000000000000000000000000000000000000000000000000000002,
                                                            ),
                                                            typ: Z3,
                                                        },
                                                        BitVec {
                                                            inner: Z3(
                                                                (bvadd a #x0000000000000000000000000000000000000000000000000000000000000001),
                                                            ),
                                                            typ: Z3,
                                                        },
                                                        BitVec {
                                                            inner: Z3(
                                                                #x0000000000000000000000000000000000000000000000000000000000000007,
                                                            ),
                                                            typ: Z3,
                                                        },
                                                    ],
                                                },
                                                pc: 5,
                                                pgm: [
                                                    Push32(
                                                        BitVec {
                                                            inner: Z3(
                                                                #x0000000000000000000000000000000000000000000000000000000000000002,
                                                            ),
                                                            typ: Z3,
                                                        },
                                                    ),
                                                    Push32(
                                                        BitVec {
                                                            inner: Z3(
                                                                #x0000000000000000000000000000000000000000000000000000000000000001,
                                                            ),
                                                            typ: Z3,
                                                        },
                                                    ),
                                                    Push32(
                                                        BitVec {
                                                            inner: Z3(
                                                                a,
                                                            ),
                                                            typ: Z3,
                                                        },
                                                    ),
                                                    Add,
                                                    Push32(
                                                        BitVec {
                                                            inner: Z3(
                                                                #x0000000000000000000000000000000000000000000000000000000000000007,
                                                            ),
                                                            typ: Z3,
                                                        },
                                                    ),
                                                    JumpI,
                                                    Push(
                                                        BitVec {
                                                            inner: Z3(
                                                                #x0000000000000000000000000000000000000000000000000000000000000064,
                                                            ),
                                                            typ: Z3,
                                                        },
                                                    ),
                                                    Push32(
                                                        BitVec {
                                                            inner: Z3(
                                                                #x0000000000000000000000000000000000000000000000000000000000000032,
                                                            ),
                                                            typ: Z3,
                                                        },
                                                    ),
                                                ],
                                            },
                                            path_condition: None,
                                            left: Some(
                                                StateTree {
                                                    id: NodeId {
                                                        id: 194b94b65ac34df1bb5dc6d6a2295ec1,
                                                        parent: None,
                                                    },
                                                    val: EvmState {
                                                        memory: Memory {
                                                            inner: [],
                                                        },
                                                        stack: Stack {
                                                            stack: [
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ],
                                                        },
                                                        pc: 6,
                                                        pgm: [
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000001,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        a,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            Add,
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000007,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            JumpI,
                                                            Push(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000064,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000032,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                    path_condition: Some(
                                                        (and (not (not (= (bvadd a
                                                                                 #x0000000000000000000000000000000000000000000000000000000000000001)
                                                                          #x0000000000000000000000000000000000000000000000000000000000000000)))),
                                                    ),
                                                    left: Some(
                                                        StateTree {
                                                            id: NodeId {
                                                                id: 5ffdf966e9d1476689c4e71cab83e9e6,
                                                                parent: None,
                                                            },
                                                            val: EvmState {
                                                                memory: Memory {
                                                                    inner: [],
                                                                },
                                                                stack: Stack {
                                                                    stack: [
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000064,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ],
                                                                },
                                                                pc: 7,
                                                                pgm: [
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000001,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                a,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    Add,
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000007,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    JumpI,
                                                                    Push(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000064,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000032,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                ],
                                                            },
                                                            path_condition: Some(
                                                                (and (not (not (= (bvadd a
                                                                                         #x0000000000000000000000000000000000000000000000000000000000000001)
                                                                                  #x0000000000000000000000000000000000000000000000000000000000000000)))),
                                                            ),
                                                            left: Some(
                                                                StateTree {
                                                                    id: NodeId {
                                                                        id: cb50266a2e174d318063fe6585098087,
                                                                        parent: None,
                                                                    },
                                                                    val: EvmState {
                                                                        memory: Memory {
                                                                            inner: [],
                                                                        },
                                                                        stack: Stack {
                                                                            stack: [
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        #x0000000000000000000000000000000000000000000000000000000000000064,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        #x0000000000000000000000000000000000000000000000000000000000000032,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                            ],
                                                                        },
                                                                        pc: 8,
                                                                        pgm: [
                                                                            Push32(
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                            ),
                                                                            Push32(
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        #x0000000000000000000000000000000000000000000000000000000000000001,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                            ),
                                                                            Push32(
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        a,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                            ),
                                                                            Add,
                                                                            Push32(
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        #x0000000000000000000000000000000000000000000000000000000000000007,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                            ),
                                                                            JumpI,
                                                                            Push(
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        #x0000000000000000000000000000000000000000000000000000000000000064,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                            ),
                                                                            Push32(
                                                                                BitVec {
                                                                                    inner: Z3(
                                                                                        #x0000000000000000000000000000000000000000000000000000000000000032,
                                                                                    ),
                                                                                    typ: Z3,
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    path_condition: Some(
                                                                        (and (not (not (= (bvadd a
                                                                                                 #x0000000000000000000000000000000000000000000000000000000000000001)
                                                                                          #x0000000000000000000000000000000000000000000000000000000000000000)))),
                                                                    ),
                                                                    left: None,
                                                                    right: None,
                                                                },
                                                            ),
                                                            right: None,
                                                        },
                                                    ),
                                                    right: None,
                                                },
                                            ),
                                            right: Some(
                                                StateTree {
                                                    id: NodeId {
                                                        id: 349b98b471ad4c908ea348e334a3132b,
                                                        parent: None,
                                                    },
                                                    val: EvmState {
                                                        memory: Memory {
                                                            inner: [],
                                                        },
                                                        stack: Stack {
                                                            stack: [
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ],
                                                        },
                                                        pc: 7,
                                                        pgm: [
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000001,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        a,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            Add,
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000007,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            JumpI,
                                                            Push(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000064,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                            Push32(
                                                                BitVec {
                                                                    inner: Z3(
                                                                        #x0000000000000000000000000000000000000000000000000000000000000032,
                                                                    ),
                                                                    typ: Z3,
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                    path_condition: Some(
                                                        (and (not (= (bvadd a
                                                                            #x0000000000000000000000000000000000000000000000000000000000000001)
                                                                     #x0000000000000000000000000000000000000000000000000000000000000000))),
                                                    ),
                                                    left: Some(
                                                        StateTree {
                                                            id: NodeId {
                                                                id: 84c6c28a405c4b4b9cba4716a2e6014c,
                                                                parent: None,
                                                            },
                                                            val: EvmState {
                                                                memory: Memory {
                                                                    inner: [],
                                                                },
                                                                stack: Stack {
                                                                    stack: [
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000032,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ],
                                                                },
                                                                pc: 8,
                                                                pgm: [
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000002,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000001,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                a,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    Add,
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000007,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    JumpI,
                                                                    Push(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000064,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                    Push32(
                                                                        BitVec {
                                                                            inner: Z3(
                                                                                #x0000000000000000000000000000000000000000000000000000000000000032,
                                                                            ),
                                                                            typ: Z3,
                                                                        },
                                                                    ),
                                                                ],
                                                            },
                                                            path_condition: Some(
                                                                (and (not (= (bvadd a
                                                                                    #x0000000000000000000000000000000000000000000000000000000000000001)
                                                                             #x0000000000000000000000000000000000000000000000000000000000000000))),
                                                            ),
                                                            left: None,
                                                            right: None,
                                                        },
                                                    ),
                                                    right: None,
                                                },
                                            ),
                                        },
                                    ),
                                    right: None,
                                },
                            ),
                            right: None,
                        },
                    ),
                    right: None,
                },
            ),
            right: None,
        },
    ),
    right: None,
}
```
