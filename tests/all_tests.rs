mod infra;

// Your tests go here!
success_tests! {
    {
        name: make_vec_succ,
        file: "make_vec.snek",
        input: "5",
        expected: "[0, 0, 0, 0, 0]",
    },
    {
        name: vec_succ,
        file: "vec.snek",
        expected: "[0, 1, 2, 3]",
    },
    {
        name: vec_get_succ,
        file: "vec_get.snek",
        input: "3",
        expected: "3",
    }
}

runtime_error_tests! {
    {
        name: make_vec_oom,
        file: "make_vec.snek",
        input: "5",
        heap_size: 5,
        expected: "out of memory",
    },
    {
        name: vec_get_oob,
        file: "vec_get.snek",
        input: "5",
        expected: "",
    }
}

static_error_tests! {}
