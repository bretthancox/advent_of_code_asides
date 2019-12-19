use std::collections::HashMap;

#[derive(Clone)]
pub struct Day6Inputs<'a> {
    day6_input: Vec<&'a str>,
}


fn chase_the_longest_path<'b>(connections: &'b HashMap<&str, Vec<&str>>, key_to_use: &'b str) -> usize {
    let starting_point: Vec<&str> = connections.get(key_to_use).unwrap().to_vec();
    let mut persistent_counter: usize = starting_point.len();
    for value in starting_point {
        let temp_counter = chase_the_longest_path(connections, value);
        persistent_counter = persistent_counter + temp_counter;
    }
    persistent_counter
}


fn chase_the_longest_path_2<'b>(connections: &'b HashMap<&str, Vec<&'b str>>, key_to_use_me: &'b str, key_to_use_san: &'b str, mut me_vec: &'b mut Vec<&'b str>, mut san_vec: &'b mut Vec<&'b str>) -> (usize, usize) {
    let me_starting_point: Vec<&str> = connections.get(key_to_use_me).unwrap().to_vec();
    let mut me_persistent_counter: usize = me_starting_point.len();

    let san_starting_point: Vec<&str> = connections.get(key_to_use_san).unwrap().to_vec();
    let mut san_persistent_counter: usize = san_starting_point.len();

    
    for me_value in &me_starting_point {
        me_vec.push(me_value);
        println!("ME VEC: {:?}", me_vec);
        if san_starting_point.len() > 0 {
            chase_the_longest_path_2(connections, me_value, san_starting_point[0], &mut me_vec, &mut san_vec);}
    }
    for san_value in &san_starting_point {
        san_vec.push(san_value);
        println!("SAN_VEC: {:?}", san_vec);
        if me_starting_point.len() > 0 {
            chase_the_longest_path_2(connections, me_starting_point[0], san_value, &mut me_vec, &mut san_vec);}
          /*  println!("Starting point vec: {:?}, {:?}", me_starting_point, san_starting_point);
            println!{"Me: {} Santa: {}", me_value, san_value};
            let temp_counter = chase_the_longest_path_2(connections, me_value, san_value);
            me_persistent_counter += temp_counter.0;
            san_persistent_counter += temp_counter.1;*/
            
    }
    
    let persistent_counts: (usize, usize) = (1,1);
    persistent_counts
}



pub fn day_six() {
    let day6 = Day6Inputs {day6_input: vec!["Z9G)Q7N","D1W)7G9","FP7)F5P","L8W)55T","YZC)1HN","1DQ)QKD","4RD)38D","8V7)DC8","95Q)33Q","Y8N)G7L","X25)7YN","5QH)JWW","ZWL)HGG","H4Y)RWG","N38)XWR","PWP)F6Y","6XB)QX1","G3W)PKM","5G5)439","V1Z)CBZ","8X1)5R5","6LJ)QHD","V9Q)N7G","WRP)XBW","61Z)7KN","FQR)371","KD8)GSS","FMM)B8R","QXD)4ZG","7JC)5SB","VGH)XCZ","VSN)C2H","W76)12B","R4Y)W7R","L8H)V1Z","BHB)1T9","7S2)1V6","338)MYS","ZM3)RN1","3P8)WW7","GVG)FPQ","43B)HKJ","QN3)KK9","728)KSW","QBV)J45","XTS)WW1","Q2C)HM8","X25)9TL","V63)WR1","DP1)PX5","GNV)J4Y","672)BY5","CK1)WD2","5R5)TZR","4JW)MKF","KMR)Y8K","HDY)YZC","CGS)ZL4","RCT)238","ZNZ)J3L","17F)8LJ","YG5)YVS","1Z9)CK5","HFN)GZV","V15)97D","5NP)6XB","K9D)J32","Q36)KN8","91X)85T","XRY)LLV","ZL4)CK2","JLC)DLX","99Q)8LK","P3R)X6R","ZNZ)SH8","H7C)669","V7K)43B","Y9T)7VF","X5L)54F","RLX)TKF","5XJ)H69","7FB)MXS","D74)24S","MDV)VSN","C1Y)J7T","P8Q)BPJ","LF7)JCM","77T)XYQ","JYP)DCX","WZ6)GB3","4YN)1Z9","5FS)Y9Z","HXD)MWF","XLS)CRC","C93)9SN","STQ)3FL","D81)NLP","T7T)8V7","K6N)YSF","7ZB)73P","8NT)28S","F1F)1L8","1HN)WL1","TWS)D1W","6WG)2DS","FPN)FBN","CK2)NW8","2RS)DG7","N6F)G5L","CCK)C6M","7B9)KRX","TXD)7FB","J45)C32","CG2)V7K","6YL)S4C","PVN)CX9","BTW)4FQ","6SL)CC5","S97)3GM","47N)R5J","T9R)5NP","3FL)HXF","HF1)DSX","RWJ)33Z","NGS)JRF","T74)D5R","9SQ)747","Z7S)S4D","WHS)XQ2","PTF)1PW","KW2)RCT","4Z7)K9D","9TV)N5N","GJP)91Y","RQB)WS7","14X)Y9T","D1W)6ZH","Z5M)M52","HMM)JY4","H64)SAN","V3N)FT8","1ZQ)FTD","WJG)516","YQP)MK9","79F)GMS","NC9)CKH","VMD)W32","GMS)385","XPP)RDH","JCM)8MB","ZKH)CMV","HYY)SK7","NYN)JDY","FTD)6WG","G8K)Z4P","5NK)RRG","GKY)3Z4","Z9W)BWG","LS6)3D4","HZF)X5L","3NV)9TG","2HB)GLS","8N9)842","T95)S6F","4PZ)338","NC9)HYS","VGM)VBR","X9G)855","GGP)WTG","DRJ)HFY","SXT)49K","71R)P99","49K)4VN","B4P)G3W","DS1)CC6","PTX)Z69","19D)2V8","2DM)2VY","CNT)CH4","WD2)L5Y","VFV)NGN","BV6)8MQ","HMS)NJ1","V39)355","8B6)63R","N6V)N8S","X9X)BPV","H5L)HW5","QX1)8XJ","BGB)2XW","8P1)99V","3H8)9TV","Z2Q)XDQ","385)QS2","3D4)FMM","BL3)J3T","L6P)YG5","JV4)6SL","VL5)N6F","BPV)SG7","HLX)LB6","KSW)SSF","HKJ)KW2","VYP)YGK","4RZ)KQG","X8Q)X25","CM4)XF4","ZSG)C9Q","XXP)CG2","9LS)YRN","L4W)9SZ","98B)XJQ","C9W)HMS","SSF)9SQ","21W)J3F","4PH)H2J","PKM)SHR","35M)G4D","8FT)9LH","3RW)VX8","SSX)VZG","BHH)BL3","RTD)RNZ","RD6)KMT","YSL)1N8","X1B)BH8","LHB)HNN","73P)LHB","TGY)Z2L","JPD)HK3","SN6)BTD","FW3)T7T","CBZ)1PM","HV3)CM4","2DP)TX4","6P4)FWL","HNN)BDH","51S)YS5","3VP)GZC","3D4)DDR","MWF)66Z","M52)TWS","7XM)V9Q","XQ2)KQH","4XN)H8Y","QKD)FP7","FPQ)VC3","91Y)16Y","752)6P7","RZN)T3D","YS5)Z7S","V1D)19X","14P)7WH","KQT)FLS","WVC)ZSG","JXC)4JJ","B8R)KD8","6S7)2MS","NJH)S75","S75)4Z7","8LJ)Y19","C5K)57N","SH2)HSR","CJJ)13S","1HN)WVJ","H3M)HLX","N1R)Z9W","S4C)XLS","HP7)7LK","WVJ)7JX","GY3)SKW","HSP)8X1","VX8)2JT","S4D)6CQ","9TM)752","J3L)G1K","5LT)G4W","HFY)6SN","4X9)VGY","VMD)YK2","66Z)PWP","HGP)YM7","JDY)BDM","ZSF)P2V","MK9)XXP","JRF)RFM","RNZ)QRW","7QF)NJY","BWN)YD5","9SS)X9V","H69)GNV","ZZH)LL9","CM7)NYN","J24)BGB","RY9)5QL","CCK)H5L","J5M)DRJ","7LK)T74","373)RZN","WQ1)7JC","DG7)QBV","LSR)PMX","6ZH)PYL","MFL)4X9","LTB)Z5M","36D)P7S","63P)DP1","38D)DVK","D2P)XL6","1SZ)T3Q","N27)F1F","F9Q)BV6","ZDS)SDJ","JW6)61Z","7YN)Y14","BXV)9LS","JY4)VT4","GVG)8FT","HJW)3RW","K39)GYC","14F)NCB","MXS)2DM","24S)98N","TKZ)CJJ","XCZ)3NV","NLP)X1B","87T)RL7","JBF)SQV","3MZ)R56","7JX)ZNM","2S8)MPH","MTL)LVS","M87)3W9","HXF)2L3","751)WG5","4ZG)65P","28R)DS1","LPT)HYY","ZGT)HKP","W63)QXH","ZL6)C4K","W66)LVF","QSN)9BR","8LY)BTW","TP7)YTG","SWG)BJ2","8MB)373","CS8)ZDS","34F)N1R","P1S)W66","BWG)H2K","G9T)ZM3","DHJ)672","H6G)V2T","FCY)S1M","XVM)4WM","N6F)N27","LZ3)HDY","9D8)6P4","2L3)PSK","5HS)SH3","4FQ)MDQ","QXD)JPD","6H1)2JH","GNV)WXS","RRG)Z2Q","SQV)HQ1","GCN)JL5","LG6)SH2","ZXK)PTF","2JT)3H8","SKW)HMM","Q46)T95","2XW)71R","DBL)35T","T7D)NT9","PJ7)PMC","M6X)DBK","PZT)7P1","G9H)5LT","1V6)D1X","MDV)M6G","WVC)9D8","T6X)NJG","Q44)SSX","BY5)Q44","Y8K)DKH","DLX)H64","NJ1)79F","3ZH)R9Z","PYP)CW6","N38)728","3SC)NBL","XWR)4RC","BPJ)X24","FCP)WQ1","8XJ)NJH","6V6)2TK","M7H)PR6","65P)XM7","9R1)D74","MBZ)VX3","XJQ)P8Q","HM8)HSP","5W2)MPV","DVK)17F","8LK)5XJ","BP2)6FD","C6M)QLL","ZXJ)KMR","8QS)YTH","W35)3R1","1N8)HKQ","4VN)H6Z","2VW)3V9","HYS)VFV","YZ3)ZK5","KNV)3VP","FWL)YSL","NRN)P38","MFX)PVN","MZ7)3P8","LLV)PYP","MGY)7H8","N26)KCC","4ZK)2DP","HJS)ZSF","HTV)JXC","3J3)W4G","4GG)JNB","X6R)HY5","GLS)FQR","9LH)H4Y","F4P)TV5","HLY)V39","HM8)LG6","F6Y)JHK","VPC)NN3","YWT)X8T","JV5)NQS","7FL)3ZH","MKY)P3R","SCR)RY9","5FR)T47","BJ2)5FR","NJY)MZ7","BTD)JLC","516)PXS","G8K)K3T","9SZ)PTX","MYS)47N","Z4P)CRS","J84)V63","6P7)VYP","YGK)596","SF4)VL5","PX5)4SJ","2V8)F4W","XSC)FDN","RBZ)35M","XL6)4KJ","K9M)R47","Q7N)LZ3","DJ2)Q36","HY5)X8Q","35T)PHH","C3Q)6V6","SH3)CTS","X8T)DHJ","3K4)W76","LF1)KTP","GB3)2RS","RL7)HLL","G1K)9TM","63R)F4P","LR9)Q2C","XM7)6YL","9TG)P67","C32)4GG","5NP)L1Z","3V9)1TL","GJP)ZGT","N5N)L71","VX3)7C8","Q8Q)95Q","Z3T)1N1","57N)HJS","QS2)3SC","PV8)8FF","63P)8QS","PBV)JV5","KQG)TXL","7P1)X9X","ZK5)M28","JVF)P1S","DL8)JTY","9D8)91X","4WT)XCJ","S6F)TPQ","FDN)KPT","KS4)2VW","NGN)7NQ","WW1)XSC","HP7)YOU","KTP)MV8","49K)2VD","2CJ)CCK","J4Y)YSY","Q2Q)MWV","YTH)J5M","26H)CXL","MWV)SCR","W4G)9TT","Y19)K6N","5QL)GKY","R5K)BBQ","C2H)M74","WW7)Q8Q","YSF)J84","9BR)14P","6FD)CM7","7G9)XTS","TV5)ZKK","X24)Y8N","CSK)3YD","HSR)H3M","211)CK1","92P)BHH","MGZ)MDV","5ZR)Q2Q","P7S)2FV","JWW)Z9G","99V)ZZH","WXS)S97","PVN)HP7","6CQ)KXK","RZF)RD6","M87)RLQ","Y9Z)DT2","PR6)GNR","WL1)VGH","LG6)6Z2","C4K)HV3","WS7)KD5","98N)XCT","CKH)BHB","8CH)TGY","GMS)99Q","7VF)K39","D54)LF1","4SJ)PV8","BDM)JW6","3YD)ZL6","BBQ)5NK","9BR)GCN","73L)D54","MDQ)TP7","C9Q)2S8","2TK)77T","PR5)LS6","L5Y)FHX","VZG)4PH","NCB)D31","H6Z)PBV","6YD)26H","XCT)63P","BDH)8KZ","VX3)ZNZ","7PM)3MZ","F4W)R4Y","HKQ)PZT","HKY)QN3","TQY)FCY","NJG)QX8","CMV)8NT","NN3)HK9","7P1)5ZR","697)YQP","RV3)69K","QX8)G9H","R5J)RWJ","13S)T2D","NT9)NC4","RLQ)BR6","M24)CS8","YSY)2HB","VGY)DBL","QQN)CCC","RWG)X76","SSX)F9Q","FT5)N38","6QQ)SY4","JL5)L4W","2VD)211","5BF)36D","JV4)L6P","RDH)XW8","61C)LC1","85T)FCP","4H6)LPT","CH4)F8G","BB7)RZF","JHK)T6Z","SHR)LTB","HK9)JB4","6QQ)TQY","5WX)KQT","7G9)L8H","4T5)CNT","T3D)92P","7NQ)77J","DDR)8M3","DBK)X9G","CGQ)9J8","G5L)3WM","YVS)M6X","SG7)JK2","CK5)V1D","NQS)26M","C32)TXD","51G)FKP","X8T)5BF","XW8)6LJ","CBZ)87T","Q5J)NGW","3YD)VGM","WR1)C1Y","2MS)H6G","JK2)9R1","YM7)21W","W32)6WP","4T5)CV6","CW6)D2P","DSX)7XM","YDG)7ZB","T13)Z3T","P67)2JF","XGK)SX6","NFB)DCW","KN8)KZH","TXL)14F","KD5)HGP","2VD)S9X","TQV)D81","TRM)75P","PXS)T13","BH8)HJW","48P)2CJ","H2J)RLX","XYQ)NGS","FKP)14X","SK7)7PM","LVS)D56","DT2)J24","439)M5S","4Z7)4ZK","HGG)7MR","VC3)6QQ","3W9)DDZ","842)YWT","PHH)RV3","B3V)JVF","7C8)WHS","9TT)61C","ZNM)RTD","48Y)6S7","PRW)HV6","LC1)HTV","75P)34F","MPH)MTL","2X4)4H6","F8G)N6V","VC3)WRP","MLR)SKV","7WH)1GY","CZC)NFB","9SN)DL8","COM)HXD","XZV)9SS","YTG)4RD","6SQ)MFL","3GM)K9M","1N1)4XN","PS3)FW3","HK3)CGQ","N7G)ZWL","28S)NRN","GNR)BXQ","DKH)LSR","Z69)T6X","M74)1ZN","J3T)MLR","SH8)1ZQ","L1Z)WVC","CXL)M49","GGP)BWN","7MR)NX2","1GY)6SQ","KQH)HKY","LL9)V15","TNS)HZF","Y6X)6YD","9VN)TNS","K3T)YDG","48P)XRY","G7L)5FS","HV6)7S2","7H8)SF4","4RC)MBZ","BH8)48P","K3N)7QF","YM7)FT5","1T9)M87","PYL)DJ2","Y14)9VN","12B)LF7","C9Q)48Y","X9V)T9R","MPV)ZKH","MDQ)2X4","GVY)XGK","G4D)8ZH","T47)4PZ","M49)MGY","RN1)V3N","8KZ)K3N","211)TRM","69K)BP2","7LK)PS3","NX2)6H1","D5R)GJP","S9X)PJ7","D1X)Y6X","GZC)19D","8M3)QQN","YNN)PR5","33Q)5QH","YK2)GVG","4JJ)751","GSS)5G5","MKF)KNV","3WM)VQH","QSX)VPC","FLS)MKY","MKF)4RZ","33Z)BXV","CC5)FPN","RJT)28R","YD5)TVW","H2K)HF1","TKZ)1SZ","Z4Y)JBF","X76)FJW","VT4)4YN","Q44)M7H","L71)M24","RBN)CS1","DLL)M81","DDZ)G8K","1ZN)Z4Y","KK9)8CH","TZR)JV4","9Z4)GY3","596)BB7","WG5)JYP","RDH)VMD","JNB)C5K","XF4)GGP","SKV)NJS","FBN)71F","9J8)1KP","NBL)9Z4","6Z2)L8W","16Y)N26","DC8)4WT","FT8)4JW","97D)8LY","NDH)QSN","VQH)LR9","PMX)3Y2","T3Q)Q46","M6G)XSB","8LK)51G","J12)T7D","FHX)XVM","77J)5HS","9WM)RQB","HKP)CSK","CGH)RBZ","9TL)QSX","KCC)H7C","SDJ)SWG","SY4)YF8","2FV)C9W","Z2L)PRW","W7R)WZ6","72C)STQ","YRN)B4P","1PM)8B6","T6Z)C93","CRC)XZV","8MQ)RJT","J3F)CGS","238)8N9","19X)JN8","QXH)ZXK","6P7)X3M","ZKK)RBN","P2V)DLL","J32)3J3","NJS)8P1","6WP)MGZ","NW8)B3V","9J8)Q5J","FKP)TQV","P99)C3Q","M28)KS4","98N)9WM","CJ9)R5K","DCX)J12","2V5)QXD","JTY)SXT","HW5)XPP","2VY)W35","1KP)G2Y","747)1DQ","BR6)CGH","J7T)MFX","FJW)HLY","MGV)4T5","PMC)TKZ","S1M)7B9","KXK)5W2","GZV)DLW","YF8)72C","1L8)7FL","JB4)98B","DCW)W63","D31)DTG","CRS)ZXJ","D56)CJ9","43B)WJG","WTG)YZ3","KPT)2V5","G2Y)HFN","R9Z)3K4","7KN)51S","JN8)NC9","2JH)697","55T)73L","3R1)5WX","5SB)GVY","CX9)YNN","P38)CZC","1PW)SN6","2JF)G9T","QRW)9MD","LVF)MGV","RFM)NDH"]};
    let capture: Vec<Vec<&str>> = day6.day6_input.clone().iter().map(|x: &&str| x.split(")").collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for combination in capture {
        if connections.contains_key(combination[0]) {
            let value_vec = connections.entry(&combination[0]).or_insert(vec!["Nothing"]);
            if value_vec.iter().any(|v| v == &combination[1]) {
                println!{"Already in vector"};
            } else {
                value_vec.push(&combination[1]);
            }            
        } else {
            connections.insert(&combination[0], vec![&combination[1]]);
        }
        if !connections.contains_key(combination[1]) {
            connections.insert(&combination[1], Vec::new());
        }
    }

    let mut counter: usize = 0;

    for (key, val) in connections.iter() {
        counter += chase_the_longest_path(&connections, key);
        if val.iter().any(|n| n == &"YOU" || n == &"SAN") {
            println!("{:?}: {:?}", &key, val);
        }
    }

    println!("Total orbits: {}", counter);

    //----------PART 2------------
    let day6_2_test: Vec<&str> = vec!["COM)B","B)C","C)D","D)E","E)F","B)G","G)H","D)I","E)J","J)K","K)L","K)YOU","I)SAN"];
    //let part2_capture: Vec<Vec<&str>> = day6.day6_input.iter().map(|x: &&str| x.split(")").collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
    let part2_capture: Vec<Vec<&str>> = day6_2_test.iter().map(|x: &&str| x.split(")").collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
    let mut part2_connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for combination in part2_capture {
        if part2_connections.contains_key(combination[1]) {
            let value_vec = part2_connections.entry(&combination[1]).or_insert(vec!["Nothing"]);
            if value_vec.iter().any(|v| v == &combination[0]) {
                println!{"Already in vector"};
            } else {
                value_vec.push(&combination[0]);
            }            
        } else {
            part2_connections.insert(&combination[1], vec![&combination[0]]);
        }
        if !part2_connections.contains_key(combination[0]) {
            part2_connections.insert(&combination[0], Vec::new());
        }
    }

    println!("{:?}", part2_connections);

    
    let mut me_vec: Vec<&str> = Vec::new();
    let mut san_vec: Vec<&str> = Vec::new();

    let me_chain: (usize, usize) = chase_the_longest_path_2(&part2_connections, &"YOU", &"SAN", &mut me_vec, &mut san_vec);

    println!("{:?} {:?} {:?}", me_chain, me_vec, san_vec);

    let me_found: Option<&Vec<&str>> = connections.get(&"YOU");
    
    match me_found {
        Some(thing) => println!("{:?}", thing),
        None => println!("Not found"),
    };
    println!("{:?}", me_found);
}