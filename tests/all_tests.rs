mod infra;

// Your tests go here!
success_tests! {
    // Number and Boolean Literals
    {
        name: adder_num,
        file: "adder_num.snek",
        expected: "644",
    },
    {
        name: cobra_false_val,
        file: "cobra_false_val.snek",
        expected: "false",
    },

    // Input Expression
    {
        name: cobra_input_default,
        file: "cobra_input0.snek",
        expected: "false",
    },
    {
        name: cobra_input_bool,
        file: "cobra_input0.snek",
        input: "true",
        expected: "true",
    },
    {
        name: cobra_input_num,
        file: "cobra_input0.snek",
        input: "123",
        expected: "123",
    },

    // Simple Number Expressions
    {
        name: adder_add1,
        file: "adder_add1.snek",
        expected: "73",
    },
    {
        name: adder_add1_sub1,
        file: "adder_add1_sub1.snek",
        expected: "4",
    },
    {
        name: cobra_add_num,
        file: "cobra_add.snek",
        input: "10",
        expected: "15",
    },

    // Nested Arithmetic Expressions
    {
        name: boa_nested_arith0,
        file: "boa_nested_arith0.snek",
        expected: "35",
    },
    {
        name: boa_nested_arith1,
        file: "boa_nested_arith1.snek",
        expected: "25",
    },
    {
        name: boa_nested_arith2,
        file: "boa_nested_arith2.snek",
        expected: "0",
    },
    {
        name: cobra_nested_arith3,
        file: "cobra_nested_arith3.snek",
        input: "8",
        expected: "1117",
    },
    {
        name: boa_nested_arith4,
        file: "boa_nested_arith4.snek",
        expected: "-1",
    },

    // Dynamic Type Checks with isnum/isbool
    {
        name: cobra_type_check_succ0,
        file: "cobra_isnum.snek",
        expected: "false",
    },
    {
        name: cobra_type_check_succ1,
        file: "cobra_isnum.snek",
        input: "547",
        expected: "true",
    },
    {
        name: cobra_type_check_succ2,
        file: "cobra_isnum.snek",
        input: "true",
        expected: "false",
    },
    {
        name: cobra_type_check_succ3,
        file: "cobra_isbool.snek",
        expected: "true",
    },
    {
        name: cobra_type_check_succ4,
        file: "cobra_isbool.snek",
        input: "689",
        expected: "false",
    },
    {
        name: cobra_type_check_succ5,
        file: "cobra_type_check_succ5.snek",
        expected: "true",
    },

    // Comparison Expressions
    {
        name: cobra_compare_expr_succ0,
        file: "cobra_compare_expr_succ0.snek",
        expected: "true",
    },

    {
        name: cobra_compare_expr_succ2,
        file: "cobra_compare_expr_succ2.snek",
        expected: "true",
    },

    // Let expressions
    {
        name: boa_binding0,
        file: "boa_binding0.snek",
        expected: "5",
    },
    {
        name: boa_binding1,
        file: "boa_binding1.snek",
        expected: "-5",
    },

    {
        name: boa_binding_expr,
        file: "boa_binding_expr.snek",
        expected: "1225",
    },
    {
        name: boa_binding_nested,
        file: "boa_binding_nested.snek",
        expected: "1",
    },

    {
        name: boa_binding_chain,
        file: "boa_binding_chain.snek",
        expected: "3",
    },
    {
        name: boa_binding_nested_chain,
        file: "boa_binding_nested_chain.snek",
        expected: "12",
    },

    // Let expressions with shadowing
    {
        name: boa_shadowed_binding_succ0,
        file: "boa_shadowed_binding_succ0.snek",
        expected: "100",
    },
    {
        name: boa_shadowed_binding_succ1,
        file: "boa_shadowed_binding_succ1.snek",
        expected: "7",
    },
    {
        name: boa_shadowed_binding_succ2,
        file: "boa_shadowed_binding_succ2.snek",
        expected: "150",
    },
    {
        name: boa_shadowed_binding_succ3,
        file: "boa_shadowed_binding_succ3.snek",
        expected: "5",
    },
    {
        name: boa_shadowed_binding_succ4,
        file: "boa_shadowed_binding_succ4.snek",
        expected: "18",
    },
    {
        name: boa_shadowed_binding_succ5,
        file: "boa_shadowed_binding_succ5.snek",
        expected: "5",
    },
    {
        name: boa_shadowed_binding_succ6,
        file: "boa_shadowed_binding_succ6.snek",
        expected: "3",
    },
    {
        name: cobra_shadowed_binding_succ7,
        file: "cobra_shadowed_binding_succ7.snek",
        expected: "200",
    },

    // Misc complex expressions with arithmetic and let bindings
    {
        name: boa_complex_expr,
        file: "boa_complex_expr.snek",
        expected: "6",
    },
    {
        name: boa_quick_brown_fox,
        file: "boa_quick_brown_fox.snek",
        expected: "-3776",
    },

    // If expressions
    {
        name: cobra_if_expr_succ0,
        file: "cobra_if_expr_succ0.snek",
        expected: "10",
    },
    {
        name: cobra_if_expr_succ1,
        file: "cobra_if_expr_input.snek",
        input: "635",
        expected: "20",
    },
    {
        name: cobra_if_expr_succ2,
        file: "cobra_if_expr_succ2.snek",
        expected: "8",
    },
    {
        name: cobra_if_expr_succ3,
        file: "cobra_if_expr_succ3.snek",
        expected: "7",
    },

    // Set expr
    {
        name: cobra_set_expr_succ0,
        file: "cobra_set_expr1.snek",
        expected: "true",
    },
    {
        name: cobra_set_expr_succ1,
        file: "cobra_set_expr2.snek",
        expected: "25",
    },
    {
        name: cobra_set_expr_succ2,
        file: "cobra_set_expr3.snek",
        input: "25",
        expected: "true",
    },
    {
        name: cobra_set_expr_succ3,
        file: "cobra_set_expr3.snek",
        input: "20",
        expected: "false",
    },

    {
        name: cobra_loop_expr_succ0,
        file: "cobra_loop_expr0.snek",
        input: "3",
        expected: "6",
    },
    {
        name: cobra_loop_expr_succ1,
        file: "cobra_loop_expr0.snek",
        input: "7",
        expected: "5040",
    },
    {
        name: cobra_loop_expr_succ2,
        file: "cobra_loop_expr1.snek",
        expected: "-6",
    },
    // Functions and calls
    {
        name: diamondback_odd_even1,
        file: "diamondback_fun_mutual_recursion.snek",
        input: "4",
        expected: "true",
    },
    {
        name: diamondback_odd_even2,
        file: "diamondback_fun_mutual_recursion.snek",
        input: "201",
        expected: "false",
    },
    {
        name: diamondback_fun_nested_call,
        file: "diamondback_fun_nested_call.snek",
        input: "20",
        expected: "2100",
    },
    {
        name: diamondback_fun_two_args,
        file: "diamondback_fun_two_args.snek",
        expected: "25",
    },
    {
        name: diamondback_fun_many_args,
        file: "diamondback_fun_many_args.snek",
        input: "-1",
        expected: "4294967296",
    },
    {
        name: diamondback_fun_many_calls,
        file: "diamondback_fun_many_calls.snek",
        input: "5",
        expected: "0\n5\n10\n15\n20\n25\n0",
    },
    // Printing / Function Calls
    {
        name: diamondback_fun_many_prints,
        file: "diamondback_fun_many_calls.snek",
        input: "999",
        expected: "0\n999\n1998\n2997\n3996\n4995\n5994\n6993\n7992\n8991\n9990\n10989\n11988\n12987\n13986\n14985\n15984\n16983\n17982\n18981\n19980\n20979\n21978\n22977\n23976\n24975\n25974\n26973\n27972\n28971\n29970\n30969\n31968\n32967\n33966\n34965\n35964\n36963\n37962\n38961\n39960\n40959\n41958\n42957\n43956\n44955\n45954\n46953\n47952\n48951\n49950\n50949\n51948\n52947\n53946\n54945\n55944\n56943\n57942\n58941\n59940\n60939\n61938\n62937\n63936\n64935\n65934\n66933\n67932\n68931\n69930\n70929\n71928\n72927\n73926\n74925\n75924\n76923\n77922\n78921\n79920\n80919\n81918\n82917\n83916\n84915\n85914\n86913\n87912\n88911\n89910\n90909\n91908\n92907\n93906\n94905\n95904\n96903\n97902\n98901\n99900\n100899\n101898\n102897\n103896\n104895\n105894\n106893\n107892\n108891\n109890\n110889\n111888\n112887\n113886\n114885\n115884\n116883\n117882\n118881\n119880\n120879\n121878\n122877\n123876\n124875\n125874\n126873\n127872\n128871\n129870\n130869\n131868\n132867\n133866\n134865\n135864\n136863\n137862\n138861\n139860\n140859\n141858\n142857\n143856\n144855\n145854\n146853\n147852\n148851\n149850\n150849\n151848\n152847\n153846\n154845\n155844\n156843\n157842\n158841\n159840\n160839\n161838\n162837\n163836\n164835\n165834\n166833\n167832\n168831\n169830\n170829\n171828\n172827\n173826\n174825\n175824\n176823\n177822\n178821\n179820\n180819\n181818\n182817\n183816\n184815\n185814\n186813\n187812\n188811\n189810\n190809\n191808\n192807\n193806\n194805\n195804\n196803\n197802\n198801\n199800\n200799\n201798\n202797\n203796\n204795\n205794\n206793\n207792\n208791\n209790\n210789\n211788\n212787\n213786\n214785\n215784\n216783\n217782\n218781\n219780\n220779\n221778\n222777\n223776\n224775\n225774\n226773\n227772\n228771\n229770\n230769\n231768\n232767\n233766\n234765\n235764\n236763\n237762\n238761\n239760\n240759\n241758\n242757\n243756\n244755\n245754\n246753\n247752\n248751\n249750\n250749\n251748\n252747\n253746\n254745\n255744\n256743\n257742\n258741\n259740\n260739\n261738\n262737\n263736\n264735\n265734\n266733\n267732\n268731\n269730\n270729\n271728\n272727\n273726\n274725\n275724\n276723\n277722\n278721\n279720\n280719\n281718\n282717\n283716\n284715\n285714\n286713\n287712\n288711\n289710\n290709\n291708\n292707\n293706\n294705\n295704\n296703\n297702\n298701\n299700\n300699\n301698\n302697\n303696\n304695\n305694\n306693\n307692\n308691\n309690\n310689\n311688\n312687\n313686\n314685\n315684\n316683\n317682\n318681\n319680\n320679\n321678\n322677\n323676\n324675\n325674\n326673\n327672\n328671\n329670\n330669\n331668\n332667\n333666\n334665\n335664\n336663\n337662\n338661\n339660\n340659\n341658\n342657\n343656\n344655\n345654\n346653\n347652\n348651\n349650\n350649\n351648\n352647\n353646\n354645\n355644\n356643\n357642\n358641\n359640\n360639\n361638\n362637\n363636\n364635\n365634\n366633\n367632\n368631\n369630\n370629\n371628\n372627\n373626\n374625\n375624\n376623\n377622\n378621\n379620\n380619\n381618\n382617\n383616\n384615\n385614\n386613\n387612\n388611\n389610\n390609\n391608\n392607\n393606\n394605\n395604\n396603\n397602\n398601\n399600\n400599\n401598\n402597\n403596\n404595\n405594\n406593\n407592\n408591\n409590\n410589\n411588\n412587\n413586\n414585\n415584\n416583\n417582\n418581\n419580\n420579\n421578\n422577\n423576\n424575\n425574\n426573\n427572\n428571\n429570\n430569\n431568\n432567\n433566\n434565\n435564\n436563\n437562\n438561\n439560\n440559\n441558\n442557\n443556\n444555\n445554\n446553\n447552\n448551\n449550\n450549\n451548\n452547\n453546\n454545\n455544\n456543\n457542\n458541\n459540\n460539\n461538\n462537\n463536\n464535\n465534\n466533\n467532\n468531\n469530\n470529\n471528\n472527\n473526\n474525\n475524\n476523\n477522\n478521\n479520\n480519\n481518\n482517\n483516\n484515\n485514\n486513\n487512\n488511\n489510\n490509\n491508\n492507\n493506\n494505\n495504\n496503\n497502\n498501\n499500\n500499\n501498\n502497\n503496\n504495\n505494\n506493\n507492\n508491\n509490\n510489\n511488\n512487\n513486\n514485\n515484\n516483\n517482\n518481\n519480\n520479\n521478\n522477\n523476\n524475\n525474\n526473\n527472\n528471\n529470\n530469\n531468\n532467\n533466\n534465\n535464\n536463\n537462\n538461\n539460\n540459\n541458\n542457\n543456\n544455\n545454\n546453\n547452\n548451\n549450\n550449\n551448\n552447\n553446\n554445\n555444\n556443\n557442\n558441\n559440\n560439\n561438\n562437\n563436\n564435\n565434\n566433\n567432\n568431\n569430\n570429\n571428\n572427\n573426\n574425\n575424\n576423\n577422\n578421\n579420\n580419\n581418\n582417\n583416\n584415\n585414\n586413\n587412\n588411\n589410\n590409\n591408\n592407\n593406\n594405\n595404\n596403\n597402\n598401\n599400\n600399\n601398\n602397\n603396\n604395\n605394\n606393\n607392\n608391\n609390\n610389\n611388\n612387\n613386\n614385\n615384\n616383\n617382\n618381\n619380\n620379\n621378\n622377\n623376\n624375\n625374\n626373\n627372\n628371\n629370\n630369\n631368\n632367\n633366\n634365\n635364\n636363\n637362\n638361\n639360\n640359\n641358\n642357\n643356\n644355\n645354\n646353\n647352\n648351\n649350\n650349\n651348\n652347\n653346\n654345\n655344\n656343\n657342\n658341\n659340\n660339\n661338\n662337\n663336\n664335\n665334\n666333\n667332\n668331\n669330\n670329\n671328\n672327\n673326\n674325\n675324\n676323\n677322\n678321\n679320\n680319\n681318\n682317\n683316\n684315\n685314\n686313\n687312\n688311\n689310\n690309\n691308\n692307\n693306\n694305\n695304\n696303\n697302\n698301\n699300\n700299\n701298\n702297\n703296\n704295\n705294\n706293\n707292\n708291\n709290\n710289\n711288\n712287\n713286\n714285\n715284\n716283\n717282\n718281\n719280\n720279\n721278\n722277\n723276\n724275\n725274\n726273\n727272\n728271\n729270\n730269\n731268\n732267\n733266\n734265\n735264\n736263\n737262\n738261\n739260\n740259\n741258\n742257\n743256\n744255\n745254\n746253\n747252\n748251\n749250\n750249\n751248\n752247\n753246\n754245\n755244\n756243\n757242\n758241\n759240\n760239\n761238\n762237\n763236\n764235\n765234\n766233\n767232\n768231\n769230\n770229\n771228\n772227\n773226\n774225\n775224\n776223\n777222\n778221\n779220\n780219\n781218\n782217\n783216\n784215\n785214\n786213\n787212\n788211\n789210\n790209\n791208\n792207\n793206\n794205\n795204\n796203\n797202\n798201\n799200\n800199\n801198\n802197\n803196\n804195\n805194\n806193\n807192\n808191\n809190\n810189\n811188\n812187\n813186\n814185\n815184\n816183\n817182\n818181\n819180\n820179\n821178\n822177\n823176\n824175\n825174\n826173\n827172\n828171\n829170\n830169\n831168\n832167\n833166\n834165\n835164\n836163\n837162\n838161\n839160\n840159\n841158\n842157\n843156\n844155\n845154\n846153\n847152\n848151\n849150\n850149\n851148\n852147\n853146\n854145\n855144\n856143\n857142\n858141\n859140\n860139\n861138\n862137\n863136\n864135\n865134\n866133\n867132\n868131\n869130\n870129\n871128\n872127\n873126\n874125\n875124\n876123\n877122\n878121\n879120\n880119\n881118\n882117\n883116\n884115\n885114\n886113\n887112\n888111\n889110\n890109\n891108\n892107\n893106\n894105\n895104\n896103\n897102\n898101\n899100\n900099\n901098\n902097\n903096\n904095\n905094\n906093\n907092\n908091\n909090\n910089\n911088\n912087\n913086\n914085\n915084\n916083\n917082\n918081\n919080\n920079\n921078\n922077\n923076\n924075\n925074\n926073\n927072\n928071\n929070\n930069\n931068\n932067\n933066\n934065\n935064\n936063\n937062\n938061\n939060\n940059\n941058\n942057\n943056\n944055\n945054\n946053\n947052\n948051\n949050\n950049\n951048\n952047\n953046\n954045\n955044\n956043\n957042\n958041\n959040\n960039\n961038\n962037\n963036\n964035\n965034\n966033\n967032\n968031\n969030\n970029\n971028\n972027\n973026\n974025\n975024\n976023\n977022\n978021\n979020\n980019\n981018\n982017\n983016\n984015\n985014\n986013\n987012\n988011\n989010\n990009\n991008\n992007\n993006\n994005\n995004\n996003\n997002\n998001\n0",
    },
    {
        name: diamondback_fun_no_args,
        file: "diamondback_fun_no_args.snek",
        expected: "true\nfalse\n0\n-1\n1",
    },
    {
        name: diamondback_calling_chain0,
        file: "diamondback_calling_chain0.snek",
        expected: "100\n100",
    },
    {
        name: diamondback_calling_chain1,
        file: "diamondback_calling_chain1.snek",
        expected: "100\n100",
    },
    {
        name: diamondback_conveyer_belt,
        file: "diamondback_conveyer_belt.snek",
        expected: "-50"
    },
    {
        name: diamondback_decreasing_args,
        file: "diamondback_decreasing_args.snek",
        expected: "12"
    },
    {
        name: diamondback_many_unused_functions,
        file: "diamondback_many_unused_functions.snek",
        input: "42",
        expected: "84",
    },
    {
        name: diamondback_namespaces,
        file: "diamondback_namespaces.snek",
        expected: "2"
    },

    // More complex recursive functions
    {
        name: diamondback_recursive_ackermann,
        file: "diamondback_recursive_ackermann.snek",
        expected: "61",
    },
    {
        name: diamondback_recursive_factorial,
        file: "diamondback_recursive_factorial.snek",
        expected: "1\n1\n2\n6\n24\n120\n720\n5040",
    },
    {
        name: diamondback_recursive_fibonacci,
        file: "diamondback_recursive_fibonacci.snek",
        expected: "55",
    },
}

runtime_error_tests! {
    // integer overflow
    {
        name: cobra_number_overflow_fail0,
        file: "cobra_number_overflow_fail0.snek",
        expected: "overflow",
    },
    {
        name: cobra_number_overflow_fail1,
        file: "cobra_number_overflow_fail1.snek",
        expected: "overflow",
    },
    {
        name: cobra_number_overflow_fail2,
        file: "cobra_add.snek",
        input: "4611686018427387899",
        expected: "overflow",
    },
    {
        name: cobra_number_overflow_fail3,
        file: "cobra_nested_arith3.snek",
        input: "4611686018427387890",
        expected: "overflow",
    },
    {
        name: diamondback_eventually_overflows,
        file: "diamondback_eventually_overflows.snek",
        expected: "overflow",
    },

    // type mismatch
    {
        name: cobra_invalid_argument_fail0,
        file: "cobra_invalid_argument_fail0.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail1,
        file: "cobra_invalid_argument_fail1.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail2,
        file: "cobra_invalid_argument_fail2.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail3,
        file: "cobra_invalid_argument_fail3.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail4,
        file: "cobra_invalid_argument_fail4.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail5,
        file: "cobra_invalid_argument_fail5.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail6,
        file: "cobra_invalid_argument_fail6.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail7,
        file: "cobra_nested_arith3.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail8,
        file: "cobra_if_expr_input.snek",
        input: "665",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail9,
        file: "cobra_set_expr3.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail10,
        file: "cobra_loop_expr0.snek",
        input: "5",
        expected: "invalid argument",
    },
    {
        name: cobra_invalid_argument_fail11,
        file: "cobra_invalid_argument_fail11.snek",
        expected: "invalid argument",
    },
}

static_error_tests! {

    // Invalid S-expressions
    {
        name: boa_parse_sexp_fail1,
        file: "boa_parse_sexp_fail1.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_sexp_fail2,
        file: "boa_parse_sexp_fail2.snek",
        expected: "Invalid",
    },

    // Invalid tokens/operators
    {
        name: boa_parse_token_fail1,
        file: "boa_parse_token_fail1.snek",
        expected: "",
    },
    {
        name: boa_parse_token_fail2,
        file: "boa_parse_token_fail2.snek",
        expected: "",
    },
    {
        name: boa_parse_token_fail3,
        file: "boa_parse_token_fail3.snek",
        expected: "",
    },
    {
        name: boa_parse_token_fail4,
        file: "boa_parse_token_fail4.snek",
        expected: "",
    },


    // Invalid/Out of bounds Number Literal
    {
        name: cobra_number_bounds_fail0,
        file: "cobra_number_bounds_fail0.snek",
        expected: "Invalid",
    },
    {
        name: cobra_number_bounds_fail1,
        file: "cobra_number_bounds_fail1.snek",
        expected: "Invalid",
    },

    // Invalid operator arguments
    {
        name: boa_parse_op_fail1,
        file: "boa_parse_op_fail1.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_op_fail2,
        file: "boa_parse_op_fail2.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_op_fail3,
        file: "boa_parse_op_fail3.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_op_fai4,
        file: "boa_parse_op_fail4.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_op_fail5,
        file: "boa_parse_op_fail5.snek",
        expected: "",
    },
    {
        name: cobra_parse_op_fail6,
        file: "cobra_parse_op_fail6.snek",
        expected: "Invalid",
    },
    {
        name: cobra_parse_op_fail7,
        file: "cobra_parse_op_fail7.snek",
        expected: "Invalid",
    },
    {
        name: cobra_parse_op_fail8,
        file: "cobra_parse_op_fail8.snek",
        expected: "Invalid",
    },

    // Invalid let expressions
    {
        name: boa_parse_let_nobindings_fail,
        file: "boa_parse_let_nobindings_fail.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail1,
        file: "boa_parse_let_improperargs_fail1.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail2,
        file: "boa_parse_let_improperargs_fail2.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail3,
        file: "boa_parse_let_improperargs_fail3.snek",
        expected: "",
    },
    {
        name: boa_parse_let_improperargs_fail4,
        file: "boa_parse_let_improperargs_fail4.snek",
        expected: "Invalid",
    },
    {
        name: boa_parse_let_improperargs_fail5,
        file: "boa_parse_let_improperargs_fail5.snek",
        expected: "keyword",
    },

    {
        name: boa_duplicate_binding_fail0,
        file: "boa_duplicate_binding_fail0.snek",
        expected: "Duplicate binding",
    },
    {
        name: boa_duplicate_binding_fail1,
        file: "boa_duplicate_binding_fail1.snek",
        expected: "Duplicate binding",
    },
    {
        name: boa_duplicate_binding_fail2,
        file: "boa_duplicate_binding_fail2.snek",
        expected: "Duplicate binding",
    },

    // Invalid if expressions
    {
        name: cobra_parse_if_fail0,
        file: "cobra_parse_if_fail0.snek",
        expected: "Invalid",
    },
    {
        name: cobra_parse_if_fail1,
        file: "cobra_parse_if_fail1.snek",
        expected: "Invalid",
    },

    // Unbound identifier
    {
        name: boa_unbound_identifier_fail0,
        file: "boa_unbound_identifier_fail0.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: boa_unbound_identifier_fail1,
        file: "boa_unbound_identifier_fail1.snek",
        expected: "Unbound variable identifier y",
    },
    {
        name: boa_unbound_identifier_fail2,
        file: "boa_unbound_identifier_fail2.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: cobra_unbound_identifier_fail3,
        file: "cobra_unbound_identifier_fail3.snek",
        expected: "Unbound variable identifier z",
    },
    {
        name: cobra_unbound_identifier_fail4,
        file: "cobra_unbound_identifier_fail4.snek",
        expected: "Unbound variable identifier t",
    },
    {
        name: cobra_unbound_identifier_fail5,
        file: "cobra_unbound_identifier_fail5.snek",
        expected: "Unbound variable identifier x",
    },

    // Invalid block
    {
        name: cobra_parse_block_fail0,
        file: "cobra_parse_block_fail0.snek",
        expected: "Invalid",
    },

    // Invalid break
    {
        name: cobra_invalid_break_fail0,
        file: "cobra_invalid_break_fail0.snek",
        expected: "break",
    },

    // Invalid loop
    {
        name: cobra_invalid_loop_fail0,
        file: "cobra_invalid_loop_fail0.snek",
        expected: "Invalid",
    },
    // Invalid function
    {
        name: diamondback_fun_duplicate_parameters_fail0,
        file: "diamondback_fun_duplicate_parameters_fail0.snek",
        expected: "",
    },
    {
        name: diamondback_fun_duplicate_parameters_fail1,
        file: "diamondback_fun_duplicate_parameters_fail1.snek",
        expected: "",
    },
    {
        name: diamondback_fun_input_fail0,
        file: "diamondback_fun_input_fail0.snek",
        expected: "",
    },
    {
        name: diamondback_fun_input_fail1,
        file: "diamondback_fun_input_fail1.snek",
        expected: "",
    },
    {
        name: diamondback_fun_not_exists_fail,
        file: "diamondback_fun_not_exists_fail.snek",
        expected: "",
    },
    {
        name: diamondback_fun_wrong_numargs_fail,
        file: "diamondback_fun_wrong_numargs_fail.snek",
        expected: "",
    },
    {
        name: diamondback_fun_duplicate_names_fail,
        file: "diamondback_fun_duplicate_names_fail.snek",
        expected: "",
    },

    {
        name: diamondback_not_fun_fail0,
        file: "diamondback_not_fun_fail0.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail1,
        file: "diamondback_not_fun_fail1.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail2,
        file: "diamondback_not_fun_fail2.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail3,
        file: "diamondback_not_fun_fail3.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail4,
        file: "diamondback_not_fun_fail4.snek",
        expected: "Invalid",
    },
    {
        name: diamondback_not_fun_fail5,
        file: "diamondback_not_fun_fail5.snek",
        expected: "",
    },

    {
        name: diamondback_no_expr_fail,
        file: "diamondback_no_expr_fail.snek",
        expected: "",
    },
    {
        name: diamondback_nested_fun_fail,
        file: "diamondback_nested_fun_fail.snek",
        expected: "",
    },

    {
        name: diamondback_fun_scope_fail0,
        file: "diamondback_fun_scope_fail0.snek",
        expected: "",
    },
    {
        name: diamondback_fun_scope_fail1,
        file: "diamondback_fun_scope_fail1.snek",
        expected: "",
    },
    {
        name: diamondback_fun_scope_fail2,
        file: "diamondback_fun_scope_fail2.snek",
        expected: "",
    },
    {
        name: diamondback_fun_scope_fail3,
        file: "diamondback_fun_scope_fail3.snek",
        expected: "",
    },

    {
        name: diamondback_function_is_keyword_fail,
        file: "diamondback_function_is_keyword_fail.snek",
        expected: "",
    },
    {
        name: diamondback_function_arg_is_keyword_fail,
        file: "diamondback_function_arg_is_keyword_fail.snek",
        expected: "",
    },
}
