use macroquad::prelude::*;
pub const HENRY: [Vec2; 600] = [
    Vec2::new(-0.2225936, -0.715508),
    Vec2::new(-0.21537435, -0.7262031),
    Vec2::new(-0.20454544, -0.7262031),
    Vec2::new(-0.19131011, -0.7240641),
    Vec2::new(-0.17927808, -0.71764696),
    Vec2::new(-0.16844922, -0.70481277),
    Vec2::new(-0.15882349, -0.69625664),
    Vec2::new(-0.1516043, -0.6727273),
    Vec2::new(-0.1431818, -0.657754),
    Vec2::new(-0.13836896, -0.6470588),
    Vec2::new(-0.1359626, -0.62780744),
    Vec2::new(-0.13235295, -0.6149733),
    Vec2::new(-0.13355619, -0.63850266),
    Vec2::new(-0.1347593, -0.65347594),
    Vec2::new(-0.1359626, -0.6748662),
    Vec2::new(-0.13716573, -0.69411755),
    Vec2::new(-0.13836896, -0.7133689),
    Vec2::new(-0.14077538, -0.73475933),
    Vec2::new(-0.14077538, -0.7518716),
    Vec2::new(-0.14077538, -0.773262),
    Vec2::new(-0.14197862, -0.7925134),
    Vec2::new(-0.14197862, -0.8160428),
    Vec2::new(-0.14197862, -0.833155),
    Vec2::new(-0.14197862, -0.85026735),
    Vec2::new(-0.14197862, -0.86524063),
    Vec2::new(-0.14197862, -0.87379676),
    Vec2::new(-0.14077538, -0.85026735),
    Vec2::new(-0.13957214, -0.83101606),
    Vec2::new(-0.13716573, -0.82032084),
    Vec2::new(-0.13235295, -0.79679143),
    Vec2::new(-0.12994653, -0.775401),
    Vec2::new(-0.12633687, -0.7540107),
    Vec2::new(-0.122727275, -0.74545443),
    Vec2::new(-0.11550796, -0.7497326),
    Vec2::new(-0.108288765, -0.7540107),
    Vec2::new(-0.096256614, -0.7604277),
    Vec2::new(-0.09024066, -0.75828874),
    Vec2::new(-0.08542776, -0.7497326),
    Vec2::new(-0.080614924, -0.7368983),
    Vec2::new(-0.07580215, -0.71764696),
    Vec2::new(-0.07339573, -0.70481277),
    Vec2::new(-0.06858289, -0.6855614),
    Vec2::new(-0.06617653, -0.66844904),
    Vec2::new(-0.06256688, -0.65347594),
    Vec2::new(-0.060160458, -0.63850266),
    Vec2::new(-0.06256688, -0.6320855),
    Vec2::new(-0.064973235, -0.64491963),
    Vec2::new(-0.06617653, -0.657754),
    Vec2::new(-0.06737965, -0.67914426),
    Vec2::new(-0.06858289, -0.7005347),
    Vec2::new(-0.06978607, -0.71978605),
    Vec2::new(-0.07098931, -0.74117637),
    Vec2::new(-0.07219249, -0.7604277),
    Vec2::new(-0.07459891, -0.7775401),
    Vec2::new(-0.07459891, -0.79679143),
    Vec2::new(-0.07580215, -0.8160428),
    Vec2::new(-0.07339573, -0.83101606),
    Vec2::new(-0.06978607, -0.8417111),
    Vec2::new(-0.061363578, -0.83529407),
    Vec2::new(-0.049331546, -0.8160428),
    Vec2::new(-0.039705873, -0.79893035),
    Vec2::new(-0.02887696, -0.78395706),
    Vec2::new(-0.021657765, -0.76684487),
    Vec2::new(-0.01564169, -0.7475935),
    Vec2::new(-0.0072191954, -0.7262031),
    Vec2::new(-0.0036096573, -0.7112299),
    Vec2::new(-0.0036096573, -0.7026737),
    Vec2::new(-0.0048128366, -0.69625664),
    Vec2::new(-0.012032151, -0.69625664),
    Vec2::new(-0.019251347, -0.7026737),
    Vec2::new(-0.025267422, -0.715508),
    Vec2::new(-0.0300802, -0.73048127),
    Vec2::new(-0.033689857, -0.7497326),
    Vec2::new(-0.037299454, -0.76684487),
    Vec2::new(-0.036096275, -0.7860962),
    Vec2::new(-0.033689857, -0.79893035),
    Vec2::new(-0.02887696, -0.8139036),
    Vec2::new(-0.021657765, -0.8181817),
    Vec2::new(-0.013235271, -0.82032084),
    Vec2::new(-0.0048128366, -0.8181817),
    Vec2::new(0.0024063587, -0.80962557),
    Vec2::new(0.012032151, -0.8010695),
    Vec2::new(0.019251347, -0.78395706),
    Vec2::new(0.026470542, -0.76684487),
    Vec2::new(0.03128338, -0.7497326),
    Vec2::new(0.034893036, -0.73475933),
    Vec2::new(0.038502693, -0.7240641),
    Vec2::new(0.039705873, -0.7133689),
    Vec2::new(0.039705873, -0.73262024),
    Vec2::new(0.038502693, -0.7475935),
    Vec2::new(0.038502693, -0.7625668),
    Vec2::new(0.038502693, -0.775401),
    Vec2::new(0.039705873, -0.7882353),
    Vec2::new(0.039705873, -0.7925134),
    Vec2::new(0.040909052, -0.80748665),
    Vec2::new(0.04572189, -0.773262),
    Vec2::new(0.048128366, -0.7604277),
    Vec2::new(0.052941084, -0.7390374),
    Vec2::new(0.05895722, -0.71978605),
    Vec2::new(0.064973235, -0.70695174),
    Vec2::new(0.07219243, -0.7005347),
    Vec2::new(0.078208566, -0.70481277),
    Vec2::new(0.080614924, -0.73475933),
    Vec2::new(0.08181822, -0.76898396),
    Vec2::new(0.08422458, -0.8032084),
    Vec2::new(0.08422458, -0.8181817),
    Vec2::new(0.08903742, -0.82245976),
    Vec2::new(0.096256614, -0.8117647),
    Vec2::new(0.105882406, -0.7903743),
    Vec2::new(0.11430478, -0.76898396),
    Vec2::new(0.120320916, -0.7518716),
    Vec2::new(0.12513375, -0.73048127),
    Vec2::new(0.13114977, -0.715508),
    Vec2::new(0.13355613, -0.70481277),
    Vec2::new(0.13235295, -0.721925),
    Vec2::new(0.13235295, -0.7368983),
    Vec2::new(0.12994647, -0.7625668),
    Vec2::new(0.12633681, -0.7946523),
    Vec2::new(0.12874329, -0.8032084),
    Vec2::new(0.12994647, -0.7775401),
    Vec2::new(0.1347593, -0.7540107),
    Vec2::new(0.13957214, -0.73475933),
    Vec2::new(0.1443851, -0.71978605),
    Vec2::new(0.14919782, -0.7133689),
    Vec2::new(0.15641713, -0.715508),
    Vec2::new(0.1660428, -0.7240641),
    Vec2::new(0.17687166, -0.7262031),
    Vec2::new(0.18770051, -0.7133689),
    Vec2::new(0.19491982, -0.69625664),
    Vec2::new(0.19732618, -0.6877004),
    Vec2::new(0.19973254, -0.67058814),
    Vec2::new(0.19852948, -0.7133689),
    Vec2::new(0.196123, -0.73475933),
    Vec2::new(0.19732618, -0.7625668),
    Vec2::new(0.20454538, -0.773262),
    Vec2::new(0.21657753, -0.77967906),
    Vec2::new(0.2262032, -0.76684487),
    Vec2::new(0.23943841, -0.7497326),
    Vec2::new(0.24545455, -0.7240641),
    Vec2::new(0.25026727, -0.70481277),
    Vec2::new(0.2562834, -0.6877004),
    Vec2::new(0.2562834, -0.67914426),
    Vec2::new(0.2562834, -0.6748662),
    Vec2::new(0.25387692, -0.7005347),
    Vec2::new(0.25026727, -0.73262024),
    Vec2::new(0.25026727, -0.7497326),
    Vec2::new(0.25147045, -0.77112293),
    Vec2::new(0.25267375, -0.7946523),
    Vec2::new(0.25267375, -0.82032084),
    Vec2::new(0.25267375, -0.84812826),
    Vec2::new(0.25267375, -0.87379676),
    Vec2::new(0.24906409, -0.895187),
    Vec2::new(0.24304795, -0.91229945),
    Vec2::new(0.23703206, -0.9229947),
    Vec2::new(0.22740638, -0.9251336),
    Vec2::new(0.21657753, -0.9208555),
    Vec2::new(0.21417117, -0.90588224),
    Vec2::new(0.21778083, -0.8823529),
    Vec2::new(0.22139037, -0.86310154),
    Vec2::new(0.22860968, -0.84812826),
    Vec2::new(0.23462558, -0.833155),
    Vec2::new(0.24545455, -0.8117647),
    Vec2::new(0.25267375, -0.7946523),
    Vec2::new(0.26470578, -0.775401),
    Vec2::new(0.27312827, -0.7497326),
    Vec2::new(0.29237962, -0.73048127),
    Vec2::new(0.29959893, -0.7240641),
    Vec2::new(0.31283414, -0.7090908),
    Vec2::new(0.32125664, -0.70695174),
    Vec2::new(0.3308823, -0.70695174),
    Vec2::new(0.33449197, -0.69625664),
    Vec2::new(0.3320855, -0.6834223),
    Vec2::new(0.33328867, -0.66203207),
    Vec2::new(0.33328867, -0.6406416),
    Vec2::new(0.33328867, -0.58288765),
    Vec2::new(0.33328867, -0.55721927),
    Vec2::new(0.3308823, -0.53155077),
    Vec2::new(0.32486618, -0.48877007),
    Vec2::new(0.31764698, -0.4524063),
    Vec2::new(0.31163096, -0.41176462),
    Vec2::new(0.300802, -0.37967908),
    Vec2::new(0.29598927, -0.35401058),
    Vec2::new(0.28876996, -0.3283422),
    Vec2::new(0.28275394, -0.3048128),
    Vec2::new(0.2803476, -0.2834224),
    Vec2::new(0.27312827, -0.2663101),
    Vec2::new(0.26831543, -0.24919784),
    Vec2::new(0.2622993, -0.2577539),
    Vec2::new(0.2550801, -0.2727272),
    Vec2::new(0.24786091, -0.28770047),
    Vec2::new(0.2334224, -0.29411757),
    Vec2::new(0.21898389, -0.28556144),
    Vec2::new(0.20695186, -0.26844913),
    Vec2::new(0.19852948, -0.24491978),
    Vec2::new(0.19010699, -0.21497321),
    Vec2::new(0.18649733, -0.18288767),
    Vec2::new(0.18890369, -0.15508014),
    Vec2::new(0.19251335, -0.12085557),
    Vec2::new(0.19732618, -0.07807487),
    Vec2::new(0.2033422, -0.03957224),
    Vec2::new(0.21296787, 0.009625673),
    Vec2::new(0.22139037, 0.05240643),
    Vec2::new(0.23101604, 0.09946513),
    Vec2::new(0.2406416, 0.14224601),
    Vec2::new(0.25026727, 0.20213902),
    Vec2::new(0.25387692, 0.24919784),
    Vec2::new(0.25868976, 0.29411757),
    Vec2::new(0.25868976, 0.32192504),
    Vec2::new(0.25989294, 0.34545445),
    Vec2::new(0.25989294, 0.37326205),
    Vec2::new(0.2622993, 0.39465225),
    Vec2::new(0.2622993, 0.40962553),
    Vec2::new(0.2622993, 0.42673802),
    Vec2::new(0.2635026, 0.44171107),
    Vec2::new(0.2635026, 0.4652406),
    Vec2::new(0.2635026, 0.45454538),
    Vec2::new(0.26470578, 0.44812822),
    Vec2::new(0.26470578, 0.43743324),
    Vec2::new(0.2622993, 0.40748668),
    Vec2::new(0.2622993, 0.3775401),
    Vec2::new(0.26109624, 0.3411764),
    Vec2::new(0.2550801, 0.29625666),
    Vec2::new(0.25147045, 0.2534759),
    Vec2::new(0.24786091, 0.21497321),
    Vec2::new(0.2406416, 0.17647052),
    Vec2::new(0.23462558, 0.14866304),
    Vec2::new(0.22740638, 0.13155079),
    Vec2::new(0.21778083, 0.10160422),
    Vec2::new(0.20574868, 0.080213785),
    Vec2::new(0.19491982, 0.054545403),
    Vec2::new(0.19010699, 0.03957212),
    Vec2::new(0.18770051, 0.02887702),
    Vec2::new(0.18529415, 0.0053474903),
    Vec2::new(0.1816845, -0.026737988),
    Vec2::new(0.1804812, -0.045989335),
    Vec2::new(0.17807484, -0.07165784),
    Vec2::new(0.17927814, -0.11016041),
    Vec2::new(0.17927814, -0.12941176),
    Vec2::new(0.17927814, -0.14652407),
    Vec2::new(0.17687166, -0.17005342),
    Vec2::new(0.17687166, -0.19358283),
    Vec2::new(0.16724598, -0.21069515),
    Vec2::new(0.15882349, -0.22566843),
    Vec2::new(0.1516043, -0.23422456),
    Vec2::new(0.14197862, -0.2235294),
    Vec2::new(0.13355613, -0.20855612),
    Vec2::new(0.121524096, -0.19358283),
    Vec2::new(0.11069512, -0.17433149),
    Vec2::new(0.092647076, -0.14652407),
    Vec2::new(0.061363578, -0.10160422),
    Vec2::new(0.037299395, -0.069518685),
    Vec2::new(0.021657705, -0.035294175),
    Vec2::new(0.010828853, -0.009625673),
    Vec2::new(0.006016016, 0.013903737),
    Vec2::new(0.006016016, 0.02887702),
    Vec2::new(0.009625673, 0.048128366),
    Vec2::new(0.01564169, 0.06737971),
    Vec2::new(0.026470542, 0.08235288),
    Vec2::new(0.037299395, 0.093048096),
    Vec2::new(0.050534725, 0.10802138),
    Vec2::new(0.063770056, 0.12513363),
    Vec2::new(0.07219243, 0.13796782),
    Vec2::new(0.0830214, 0.15080214),
    Vec2::new(0.08903742, 0.16577542),
    Vec2::new(0.092647076, 0.19358277),
    Vec2::new(0.08663106, 0.21069515),
    Vec2::new(0.079411745, 0.22139037),
    Vec2::new(0.07098937, 0.23422456),
    Vec2::new(0.06256688, 0.24705875),
    Vec2::new(0.049331546, 0.24705875),
    Vec2::new(0.040909052, 0.24064171),
    Vec2::new(0.033689857, 0.23208547),
    Vec2::new(0.026470542, 0.2171123),
    Vec2::new(0.025267363, 0.1914438),
    Vec2::new(0.02767384, 0.17219245),
    Vec2::new(0.040909052, 0.15294111),
    Vec2::new(0.050534725, 0.14652407),
    Vec2::new(0.06256688, 0.14438498),
    Vec2::new(0.07700539, 0.14010692),
    Vec2::new(0.09144378, 0.15080214),
    Vec2::new(0.10467911, 0.17433155),
    Vec2::new(0.11189842, 0.19786096),
    Vec2::new(0.11550796, 0.22566843),
    Vec2::new(0.11791444, 0.24278069),
    Vec2::new(0.11791444, 0.255615),
    Vec2::new(0.11430478, 0.27058816),
    Vec2::new(0.108288765, 0.28770053),
    Vec2::new(0.09986627, 0.3048128),
    Vec2::new(0.08903742, 0.3283422),
    Vec2::new(0.07459891, 0.34331548),
    Vec2::new(0.061363578, 0.3540107),
    Vec2::new(0.048128366, 0.36042774),
    Vec2::new(0.0300802, 0.37326205),
    Vec2::new(0.013235331, 0.3775401),
    Vec2::new(-0.009625614, 0.37967908),
    Vec2::new(-0.0300802, 0.37326205),
    Vec2::new(-0.050534725, 0.3668449),
    Vec2::new(-0.06256688, 0.35614967),
    Vec2::new(-0.079411805, 0.34973264),
    Vec2::new(-0.09024066, 0.33689833),
    Vec2::new(-0.10106957, 0.32406414),
    Vec2::new(-0.108288765, 0.31122994),
    Vec2::new(-0.11189842, 0.28770053),
    Vec2::new(-0.1131016, 0.2534759),
    Vec2::new(-0.11069518, 0.22566843),
    Vec2::new(-0.10227269, 0.20000005),
    Vec2::new(-0.095053494, 0.17433155),
    Vec2::new(-0.08302134, 0.16577542),
    Vec2::new(-0.06617653, 0.15935826),
    Vec2::new(-0.051737964, 0.16577542),
    Vec2::new(-0.04090911, 0.17860961),
    Vec2::new(-0.0300802, 0.20427811),
    Vec2::new(-0.032486618, 0.23208547),
    Vec2::new(-0.04211223, 0.24919784),
    Vec2::new(-0.05895722, 0.255615),
    Vec2::new(-0.06978607, 0.255615),
    Vec2::new(-0.079411805, 0.24705875),
    Vec2::new(-0.086631, 0.2299465),
    Vec2::new(-0.095053494, 0.20427811),
    Vec2::new(-0.093850255, 0.1807487),
    Vec2::new(-0.092647076, 0.15080214),
    Vec2::new(-0.086631, 0.12727273),
    Vec2::new(-0.07580215, 0.103743315),
    Vec2::new(-0.061363578, 0.08235288),
    Vec2::new(-0.04211223, 0.06524062),
    Vec2::new(-0.025267422, 0.05240643),
    Vec2::new(-0.009625614, 0.048128366),
    Vec2::new(0.0024063587, 0.043850183),
    Vec2::new(0.0036096573, 0.026737928),
    Vec2::new(0.0024063587, 0.007486582),
    Vec2::new(-0.0036096573, -0.013903797),
    Vec2::new(-0.01443851, -0.04171127),
    Vec2::new(-0.032486618, -0.069518685),
    Vec2::new(-0.04572189, -0.08877003),
    Vec2::new(-0.05534762, -0.09518713),
    Vec2::new(-0.06858289, -0.10588235),
    Vec2::new(-0.08422458, -0.116577506),
    Vec2::new(-0.095053494, -0.1251337),
    Vec2::new(-0.10106957, -0.13368982),
    Vec2::new(-0.10347593, -0.14438504),
    Vec2::new(-0.107085526, -0.16577536),
    Vec2::new(-0.1131016, -0.19999993),
    Vec2::new(-0.122727275, -0.21497321),
    Vec2::new(-0.13355619, -0.2235294),
    Vec2::new(-0.14077538, -0.22566843),
    Vec2::new(-0.150401, -0.22139037),
    Vec2::new(-0.16002673, -0.20427805),
    Vec2::new(-0.16724598, -0.17005342),
    Vec2::new(-0.16844922, -0.14010686),
    Vec2::new(-0.17085564, -0.11871654),
    Vec2::new(-0.17566842, -0.08877003),
    Vec2::new(-0.17566842, -0.048128366),
    Vec2::new(-0.1744653, -0.01818186),
    Vec2::new(-0.16965234, 0.003208518),
    Vec2::new(-0.16483957, 0.033155084),
    Vec2::new(-0.16002673, 0.07807481),
    Vec2::new(-0.16002673, 0.11016035),
    Vec2::new(-0.16002673, 0.15294111),
    Vec2::new(-0.15882349, 0.19786096),
    Vec2::new(-0.15641707, 0.23422456),
    Vec2::new(-0.14919788, 0.27272725),
    Vec2::new(-0.14438504, 0.29625666),
    Vec2::new(-0.13355619, 0.33262026),
    Vec2::new(-0.122727275, 0.35828876),
    Vec2::new(-0.109492004, 0.38395715),
    Vec2::new(-0.09745991, 0.41176474),
    Vec2::new(-0.08302134, 0.4310161),
    Vec2::new(-0.07580215, 0.45026731),
    Vec2::new(-0.07098931, 0.47165763),
    Vec2::new(-0.07098931, 0.5016042),
    Vec2::new(-0.07700539, 0.53368974),
    Vec2::new(-0.08542776, 0.55935824),
    Vec2::new(-0.092647076, 0.5914439),
    Vec2::new(-0.10227269, 0.6149733),
    Vec2::new(-0.11430484, 0.63208556),
    Vec2::new(-0.123930454, 0.6449199),
    Vec2::new(-0.1359626, 0.6470587),
    Vec2::new(-0.14919788, 0.6449199),
    Vec2::new(-0.16002673, 0.6363635),
    Vec2::new(-0.16844922, 0.6278075),
    Vec2::new(-0.17566842, 0.6128342),
    Vec2::new(-0.18048126, 0.5957217),
    Vec2::new(-0.1816845, 0.5786096),
    Vec2::new(-0.1816845, 0.548663),
    Vec2::new(-0.17687166, 0.53368974),
    Vec2::new(-0.1660428, 0.5144385),
    Vec2::new(-0.14799464, 0.5037433),
    Vec2::new(-0.1251337, 0.5101603),
    Vec2::new(-0.11189842, 0.51871645),
    Vec2::new(-0.09144384, 0.5401069),
    Vec2::new(-0.0565508, 0.5657754),
    Vec2::new(-0.0300802, 0.59786093),
    Vec2::new(-0.0072191954, 0.6427808),
    Vec2::new(0.0024063587, 0.662032),
    Vec2::new(0.013235331, 0.6877004),
    Vec2::new(0.016844988, 0.7005347),
    Vec2::new(0.020454526, 0.7112298),
    Vec2::new(0.025267363, 0.7133689),
    Vec2::new(0.0300802, 0.6983956),
    Vec2::new(0.036096334, 0.67914426),
    Vec2::new(0.049331546, 0.6149733),
    Vec2::new(0.061363578, 0.5786096),
    Vec2::new(0.07098937, 0.5508021),
    Vec2::new(0.08422458, 0.5294118),
    Vec2::new(0.09866309, 0.5144385),
    Vec2::new(0.11430478, 0.49518704),
    Vec2::new(0.13114977, 0.4780748),
    Vec2::new(0.15762031, 0.45668435),
    Vec2::new(0.17566848, 0.4524063),
    Vec2::new(0.19852948, 0.45026731),
    Vec2::new(0.2117647, 0.46096253),
    Vec2::new(0.22259355, 0.48235285),
    Vec2::new(0.22379673, 0.5122994),
    Vec2::new(0.22259355, 0.5358288),
    Vec2::new(0.21898389, 0.5529412),
    Vec2::new(0.21417117, 0.5764706),
    Vec2::new(0.20213902, 0.5914439),
    Vec2::new(0.19010699, 0.59358287),
    Vec2::new(0.17687166, 0.59358287),
    Vec2::new(0.16363645, 0.5807487),
    Vec2::new(0.15401065, 0.57005346),
    Vec2::new(0.1431818, 0.5508021),
    Vec2::new(0.1347593, 0.53368974),
    Vec2::new(0.12633681, 0.5101603),
    Vec2::new(0.122727275, 0.48877),
    Vec2::new(0.11911762, 0.4631015),
    Vec2::new(0.11671126, 0.4395721),
    Vec2::new(0.11671126, 0.4160428),
    Vec2::new(0.11791444, 0.39679146),
    Vec2::new(0.11911762, 0.3775401),
    Vec2::new(0.11911762, 0.3518716),
    Vec2::new(0.121524096, 0.32192504),
    Vec2::new(0.12633681, 0.29411757),
    Vec2::new(0.13235295, 0.2534759),
    Vec2::new(0.14679146, 0.21283412),
    Vec2::new(0.16243315, 0.14438498),
    Vec2::new(0.1804812, 0.080213785),
    Vec2::new(0.18529415, 0.054545403),
    Vec2::new(0.18770051, 0.04171121),
    Vec2::new(0.18649733, 0.0010694265),
    Vec2::new(0.1816845, -0.03957224),
    Vec2::new(0.17927814, -0.0759359),
    Vec2::new(0.17927814, -0.12727273),
    Vec2::new(0.17687166, -0.17005342),
    Vec2::new(0.17927814, -0.20213902),
    Vec2::new(0.18890369, -0.23636353),
    Vec2::new(0.19732618, -0.2577539),
    Vec2::new(0.20454538, -0.2663101),
    Vec2::new(0.22018719, -0.28556144),
    Vec2::new(0.23462558, -0.29411757),
    Vec2::new(0.24786091, -0.28770047),
    Vec2::new(0.2550801, -0.27058816),
    Vec2::new(0.26109624, -0.25347584),
    Vec2::new(0.26470578, -0.22780746),
    Vec2::new(0.2719251, -0.1914438),
    Vec2::new(0.27553463, -0.14652407),
    Vec2::new(0.2791443, -0.114438474),
    Vec2::new(0.28275394, -0.10160422),
    Vec2::new(0.28876996, -0.08877003),
    Vec2::new(0.31042778, -0.04171127),
    Vec2::new(0.32847583, -0.011764705),
    Vec2::new(0.33449197, -0.0053476095),
    Vec2::new(0.34411752, 0.013903737),
    Vec2::new(0.35254002, 0.043850183),
    Vec2::new(0.3621657, 0.08663106),
    Vec2::new(0.36577535, 0.11016035),
    Vec2::new(0.3681817, 0.13368976),
    Vec2::new(0.3681817, 0.16363633),
    Vec2::new(0.3681817, 0.18502676),
    Vec2::new(0.3621657, 0.22352934),
    Vec2::new(0.3609625, 0.2534759),
    Vec2::new(0.35614967, 0.30695176),
    Vec2::new(0.35494637, 0.33475935),
    Vec2::new(0.35133672, 0.3647058),
    Vec2::new(0.346524, 0.39893043),
    Vec2::new(0.33689833, 0.44385016),
    Vec2::new(0.32606947, 0.48663092),
    Vec2::new(0.31524062, 0.5358288),
    Vec2::new(0.31283414, 0.59358287),
    Vec2::new(0.3092245, 0.62352943),
    Vec2::new(0.29839563, 0.6342244),
    Vec2::new(0.2935828, 0.65561485),
    Vec2::new(0.28395712, 0.6919786),
    Vec2::new(0.2719251, 0.7133689),
    Vec2::new(0.2550801, 0.7283422),
    Vec2::new(0.24545455, 0.7390374),
    Vec2::new(0.23823524, 0.7647058),
    Vec2::new(0.22860968, 0.7860962),
    Vec2::new(0.2117647, 0.8139037),
    Vec2::new(0.19371653, 0.8331549),
    Vec2::new(0.173262, 0.8481282),
    Vec2::new(0.14919782, 0.8631015),
    Vec2::new(0.12994647, 0.88449204),
    Vec2::new(0.105882406, 0.895187),
    Vec2::new(0.080614924, 0.89304805),
    Vec2::new(0.0601604, 0.89304805),
    Vec2::new(0.048128366, 0.88877),
    Vec2::new(0.038502693, 0.87807477),
    Vec2::new(0.02887702, 0.8737967),
    Vec2::new(0.021657705, 0.87807477),
    Vec2::new(0.008422494, 0.8866309),
    Vec2::new(-0.010828853, 0.8973261),
    Vec2::new(-0.032486618, 0.90802133),
    Vec2::new(-0.05775404, 0.90588224),
    Vec2::new(-0.07098931, 0.8994653),
    Vec2::new(-0.095053494, 0.89090896),
    Vec2::new(-0.122727275, 0.8716576),
    Vec2::new(-0.1431818, 0.8631015),
    Vec2::new(-0.16002673, 0.8545455),
    Vec2::new(-0.17687166, 0.8331549),
    Vec2::new(-0.19852942, 0.8032085),
    Vec2::new(-0.21537435, 0.78395724),
    Vec2::new(-0.22500002, 0.7647058),
    Vec2::new(-0.23703206, 0.74545455),
    Vec2::new(-0.24906415, 0.7112298),
    Vec2::new(-0.25868982, 0.6855614),
    Vec2::new(-0.26590908, 0.6470587),
    Vec2::new(-0.276738, 0.6128342),
    Vec2::new(-0.2863636, 0.5743315),
    Vec2::new(-0.29839575, 0.5358288),
    Vec2::new(-0.30681813, 0.49304807),
    Vec2::new(-0.31403744, 0.45668435),
    Vec2::new(-0.31524062, 0.42032075),
    Vec2::new(-0.32245988, 0.38823533),
    Vec2::new(-0.33569515, 0.34331548),
    Vec2::new(-0.34532082, 0.31336892),
    Vec2::new(-0.35254014, 0.28128338),
    Vec2::new(-0.3609625, 0.24491978),
    Vec2::new(-0.363369, 0.22139037),
    Vec2::new(-0.36457217, 0.18716574),
    Vec2::new(-0.36457217, 0.1550802),
    Vec2::new(-0.36216575, 0.116577506),
    Vec2::new(-0.3549465, 0.08877003),
    Vec2::new(-0.35013366, 0.058823466),
    Vec2::new(-0.34411764, 0.031015992),
    Vec2::new(-0.33449197, -0.0010695457),
    Vec2::new(-0.3248663, -0.026737988),
    Vec2::new(-0.31042778, -0.05454552),
    Vec2::new(-0.29719245, -0.082352936),
    Vec2::new(-0.2863636, -0.116577506),
    Vec2::new(-0.2743315, -0.1486631),
    Vec2::new(-0.26590908, -0.17860955),
    Vec2::new(-0.25989306, -0.22566843),
    Vec2::new(-0.25508022, -0.26417106),
    Vec2::new(-0.2502674, -0.28556144),
    Vec2::new(-0.24545455, -0.29197854),
    Vec2::new(-0.24064171, -0.28770047),
    Vec2::new(-0.23342246, -0.2834224),
    Vec2::new(-0.21537435, -0.2620321),
    Vec2::new(-0.20695186, -0.24278075),
    Vec2::new(-0.19852942, -0.21283418),
    Vec2::new(-0.19131011, -0.18716574),
    Vec2::new(-0.18770057, -0.17647052),
    Vec2::new(-0.18288767, -0.16149724),
    Vec2::new(-0.17927808, -0.13368982),
    Vec2::new(-0.18048126, -0.11229944),
    Vec2::new(-0.1816845, -0.07379687),
    Vec2::new(-0.18288767, -0.048128366),
    Vec2::new(-0.18529415, -0.020320892),
    Vec2::new(-0.18890369, 0.01604271),
    Vec2::new(-0.19371653, 0.058823466),
    Vec2::new(-0.19852942, 0.11229944),
    Vec2::new(-0.20454544, 0.15080214),
    Vec2::new(-0.20935827, 0.20427811),
    Vec2::new(-0.21056145, 0.23636365),
    Vec2::new(-0.21417111, 0.255615),
    Vec2::new(-0.21417111, 0.2898395),
    Vec2::new(-0.22018713, 0.33903742),
    Vec2::new(-0.2225936, 0.3647058),
    Vec2::new(-0.2225936, 0.38823533),
    Vec2::new(-0.2225936, 0.40962553),
    Vec2::new(-0.22018713, 0.4310161),
    Vec2::new(-0.2225936, 0.40106952),
    Vec2::new(-0.22018713, 0.3518716),
    Vec2::new(-0.21657753, 0.3048128),
    Vec2::new(-0.21417111, 0.26203203),
    Vec2::new(-0.20574868, 0.22139037),
    Vec2::new(-0.19491976, 0.18502676),
    Vec2::new(-0.18409091, 0.1550802),
    Vec2::new(-0.17205876, 0.12299466),
    Vec2::new(-0.16483957, 0.103743315),
    Vec2::new(-0.1660428, 0.08449197),
    Vec2::new(-0.16243315, 0.060962558),
    Vec2::new(-0.16483957, 0.035294056),
    Vec2::new(-0.16724598, 0.020320773),
    Vec2::new(-0.173262, -0.0074866414),
    Vec2::new(-0.17566842, -0.048128366),
    Vec2::new(-0.17566842, -0.07165784),
    Vec2::new(-0.173262, -0.10802138),
    Vec2::new(-0.16965234, -0.13368982),
    Vec2::new(-0.16483957, -0.18716574),
    Vec2::new(-0.1660428, -0.2235294),
    Vec2::new(-0.1660428, -0.24491978),
    Vec2::new(-0.16483957, -0.26844913),
    Vec2::new(-0.17807484, -0.34973252),
    Vec2::new(-0.19371653, -0.41176462),
    Vec2::new(-0.20695186, -0.48021376),
    Vec2::new(-0.21778071, -0.5508021),
    Vec2::new(-0.22018713, -0.60213894),
    Vec2::new(-0.22139037, -0.66203207),
];
