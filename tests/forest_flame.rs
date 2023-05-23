mod infra;

success_tests! {
    // Vectors
    subdir: "forest_flame",
    {
        name: forest_flame_reversell,
        file: "forest_flame_reversell.snek",
        input: "1001",
        expected: "[19, [18, [17, [16, [15, [14, [13, [12, [11, [10, [9, [8, [7, [6, [5, [4, [3, [2, [1, [0, [-1, [-2, [-3, [-4, [-5, nil]]]]]]]]]]]]]]]]]]]]]]]]]"
    },
    {
        name: forest_flame_make_vec,
        file: "forest_flame_make_vec.snek",
        input: "10",
        heap_size: 12,
        expected: "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"
    },
    {
        name: forest_flame_nil_eq,
        file: "forest_flame_nil_eq.snek",
        expected: "true"
    },
    {
        name: forest_flame_preorder,
        file: "forest_flame_preorder.snek",
        expected: "[4, [3, [1, [9, nil]]]]"
    },
    {
        name: forest_flame_treesum,
        file: "forest_flame_treesum.snek",
        expected: "31"
    },
    {
        name: forest_flame_is_vec_fail0,
        file: "forest_flame_input_is_vec.snek",
        input: "0",
        expected: "false"
    },
    {
        name: forest_flame_is_vec_fail1,
        file: "forest_flame_input_is_vec.snek",
        input: "1",
        expected: "false",
    },
    {
        name: forest_flame_is_vec_fail2,
        file: "forest_flame_nil_is_vec.snek",
        input: "1",
        expected: "true",
    },
    {
        name: forest_flame_is_vec_fail3,
        file: "forest_flame_vec_is_vec.snek",
        input: "1",
        expected: "true",
    },
    {
        name: forest_flame_vec_get_succ,
        file: "forest_flame_vec_get.snek",
        input: "0",
        expected: "0",
    },
    {
        name: forest_flame_vec_set_succ,
        file: "forest_flame_vec_set.snek",
        input: "0",
        expected: "[1, 0, 0]",
    },
    {
        name: forest_flame_vec1,
        file: "forest_flame_vec1.snek",
        expected: "[0, 1, 2, 3]",
    },
    {
        name: forest_flame_kmp,
        file: "forest_flame_kmp.snek",
        expected: "3",
    },
    {
        name: forest_flame_bin_search0,
        file: "forest_flame_bin_search.snek",
        input: "3",
        expected: "2",
    },
    {
        name: forest_flame_bin_search1,
        file: "forest_flame_bin_search.snek",
        input: "88",
        expected: "7",
    },
    {
        name: forest_flame_quick_sort,
        file: "forest_flame_quick_sort.snek",
        expected: "[0, 1, 3, 6, 7, 9, 9, 10, 18, 38, 49]",
    },
    {
        name: forest_flame_cbn,
        file: "forest_flame_cbn.snek",
        heap_size: 50000,
        expected: "[0, 1]",
    },
}

runtime_error_tests! {
    subdir: "forest_flame",
    // Vectors
    {
        name: forest_flame_deref_nil,
        file: "forest_flame_deref_nil.snek",
        expected: "invalid argument"
    },
    {
        name: forest_flame_vec_get_fail1,
        file: "forest_flame_vec_get.snek",
        input: "-1",
        expected: "index out of bounds",
    },
    {
        name: forest_flame_vec_get_fail2,
        file: "forest_flame_vec_get.snek",
        input: "3",
        expected: "index out of bounds",
    },
    {
        name: forest_flame_vec_set_fail1,
        file: "forest_flame_vec_set.snek",
        input: "-1",
        expected: "index out of bounds",
    },
    {
        name: forest_flame_vec_set_fail2,
        file: "forest_flame_vec_set.snek",
        input: "3",
        expected: "index out of bounds",
    },
}
