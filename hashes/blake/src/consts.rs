#![cfg_attr(rustfmt, rustfmt_skip)]

pub const PADDING: &[u8; 129] =
    b"\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
      \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
      \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

pub const SIGMA: [[u8; 16]; 16] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
];

pub const BLAKE256_U: [u32; 16] = [
    0x243f_6a88, 0x85a3_08d3, 0x1319_8a2e, 0x0370_7344,
    0xa409_3822, 0x299f_31d0, 0x082e_fa98, 0xec4e_6c89,
    0x4528_21e6, 0x38d0_1377, 0xbe54_66cf, 0x34e9_0c6c,
    0xc0ac_29b7, 0xc97c_50dd, 0x3f84_d5b5, 0xb547_0917
];

pub const BLAKE512_U: [u64; 16] = [
    0x243f_6a88_85a3_08d3, 0x1319_8a2e_0370_7344, 0xa409_3822_299f_31d0, 0x082e_fa98_ec4e_6c89,
    0x4528_21e6_38d0_1377, 0xbe54_66cf_34e9_0c6c, 0xc0ac_29b7_c97c_50dd, 0x3f84_d5b5_b547_0917,
    0x9216_d5d9_8979_fb1b, 0xd131_0ba6_98df_b5ac, 0x2ffd_72db_d01a_dfb7, 0xb8e1_afed_6a26_7e96,
    0xba7c_9045_f12c_7f99, 0x24a1_9947_b391_6cf7, 0x0801_f2e2_858e_fc16, 0x6369_20d8_7157_4e69
];

pub const BLAKE224_IV: [[u32; 4]; 2] = [
    [0xc105_9ed8, 0x367c_d507, 0x3070_dd17, 0xf70e_5939],
    [0xffc0_0b31, 0x6858_1511, 0x64f9_8fa7, 0xbefa_4fa4]
];

pub const BLAKE256_IV: [[u32; 4]; 2] = [
    [0x6a09_e667, 0xbb67_ae85, 0x3c6e_f372, 0xa54f_f53a],
    [0x510e_527f, 0x9b05_688c, 0x1f83_d9ab, 0x5be0_cd19]
];

pub const BLAKE384_IV: [[u64; 4]; 2] = [
    [0xcbbb_9d5d_c105_9ed8, 0x629a_292a_367c_d507, 0x9159_015a_3070_dd17, 0x152f_ecd8_f70e_5939],
    [0x6733_2667_ffc0_0b31, 0x8eb4_4a87_6858_1511, 0xdb0c_2e0d_64f9_8fa7, 0x47b5_481d_befa_4fa4]
];

pub const BLAKE512_IV: [[u64; 4]; 2] = [
    [0x6a09_e667_f3bc_c908, 0xbb67_ae85_84ca_a73b, 0x3c6e_f372_fe94_f82b, 0xa54f_f53a_5f1d_36f1],
    [0x510e_527f_ade6_82d1, 0x9b05_688c_2b3e_6c1f, 0x1f83_d9ab_fb41_bd6b, 0x5be0_cd19_137e_2179]
];
