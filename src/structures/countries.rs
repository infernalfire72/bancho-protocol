use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use std::io::{Error, ErrorKind, Result};
#[repr(u8)]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, BinarySerialize, BinaryDeserialize, ByteSized,
)]
#[crate_root(crate)]
pub enum Country {
    #[default]
    Unknown,                       // XX
    Oceania,                       // OC
    Europe,                        // EU
    Andorra,                       // AD
    UAE,                           // AE
    Afghanistan,                   // AF
    Antigua,                       // AG
    Anguilla,                      // AI
    Albania,                       // AL
    Armenia,                       // AM
    Antilles,                      // AN
    Angola,                        // AO
    Antarctica,                    // AQ
    Argentina,                     // AR
    AmericanSamoa,                 // AS
    Austria,                       // AT
    Australia,                     // AU
    Aruba,                         // AW
    Azerbaijan,                    // AZ
    Bosnia,                        // BA
    Barbados,                      // BB
    Bangladesh,                    // BD
    Belgium,                       // BE
    BurkinaFaso,                   // BF
    Bulgaria,                      // BG
    Bahrain,                       // BH
    Burundi,                       // BI
    Benin,                         // BJ
    Bermuda,                       // BM
    BruneiDarussalam,              // BN
    Bolivia,                       // BO
    Brazil,                        // BR
    Bahamas,                       // BS
    Bhutan,                        // BT
    BouvetIsland,                  // BV
    Botswana,                      // BW
    Belarus,                       // BY
    Belize,                        // BZ
    Canada,                        // CA
    CocosIslands,                  // CC
    DRCongo,                       // CD
    CentralAfricanRepublic,        // CF
    Congo,                         // CG
    Switzerland,                   // CH
    CoteDIvoire,                   // CI
    CookIslands,                   // CK
    Chile,                         // CL
    Cameroon,                      // CM
    China,                         // CN
    Colombia,                      // CO
    CostaRica,                     // CR
    Cuba,                          // CU
    CapeVerde,                     // CV
    ChristmasIsland,               // CX
    Cyprus,                        // CY
    CzechRepublic,                 // CZ
    Germany,                       // DE
    Djibouti,                      // DJ
    Denmark,                       // DK
    Dominica,                      // DM
    DominicanRepublic,             // DO
    Algeria,                       // DZ
    Ecuador,                       // EC
    Estonia,                       // EE
    Egypt,                         // EG
    WesternSahara,                 // EH
    Eritrea,                       // ER
    Spain,                         // ES
    Ethiopia,                      // ET
    Finland,                       // FI
    Fiji,                          // FJ
    FalklandIslands,               // FK
    Micronesia,                    // FM
    FaroeIslands,                  // FO
    France,                        // FR
    MetropolitanFrance,            // FX
    Gabon,                         // GA
    UnitedKingdom,                 // GB
    Grenada,                       // GD
    Georgia,                       // GE
    FrenchGuiana,                  // GF
    Ghana,                         // GH
    Gibraltar,                     // GI
    Greenland,                     // GL
    Gambia,                        // GM
    Guinea,                        // GN
    Guadeloupe,                    // GP
    EquatorialGuinea,              // GQ
    Greece,                        // GR
    SouthGeorgia,                  // GS
    Guatemala,                     // GT
    Guam,                          // GU
    GuineaBissau,                  // GW
    Guyana,                        // GY
    HongKong,                      // HK
    HeardIslandAndMcDonaldIslands, // HM
    Honduras,                      // HN
    Croatia,                       // HR
    Haiti,                         // HT
    Hungary,                       // HU
    Indonesia,                     // ID
    Ireland,                       // IE
    Israel,                        // IL
    India,                         // IN
    BritishIndianOceanTerritory,   // IO
    Iraq,                          // IQ
    Iran,                          // IR
    Iceland,                       // IS
    Italy,                         // IT
    Jamaica,                       // JM
    Jordan,                        // JO
    Japan,                         // JP
    Kenya,                         // KE
    Kyrgyzstan,                    // KG
    Cambodia,                      // KH
    Kiribati,                      // KI
    Comoros,                       // KM
    SaintKittsAndNevis,            // KN
    NorthKorea,                    // KP
    SouthKorea,                    // KR
    Kuwait,                        // KW
    CaymanIslands,                 // KY
    Kazakhstan,                    // KZ
    Laos,                          // LA
    Lebanon,                       // LB
    SaintLucia,                    // LC
    Liechtenstein,                 // LI
    SriLanka,                      // LK
    Liberia,                       // LR
    Lesotho,                       // LS
    Lithuania,                     // LT
    Luxembourg,                    // LU
    Latvia,                        // LV
    Libya,                         // LY
    Morocco,                       // MA
    Monaco,                        // MC
    Moldova,                       // MD
    Madagascar,                    // MG
    MarshallIslands,               // MH
    Macedonia,                     // MK
    Mali,                          // ML
    Myanmar,                       // MM
    Mongolia,                      // MN
    Macao,                         // MO
    NorthernMarianaIslands,        // MP
    Martinique,                    // MQ
    Mauritania,                    // MR
    Montserrat,                    // MS
    Malta,                         // MT
    Mauritius,                     // MU
    Maldives,                      // MV
    Malawi,                        // MW
    Mexico,                        // MX
    Malaysia,                      // MY
    Mozambique,                    // MZ
    Namibia,                       // NA
    NewCaledonia,                  // NC
    Niger,                         // NE
    NorfolkIsland,                 // NF
    Nigeria,                       // NG
    Nicaragua,                     // NI
    Netherlands,                   // NL
    Norway,                        // NO
    Nepal,                         // NP
    Nauru,                         // NR
    Niue,                          // NU
    NewZealand,                    // NZ
    Oman,                          // OM
    Panama,                        // PA
    Peru,                          // PE
    FrenchPolynesia,               // PF
    PapuaNewGuinea,                // PG
    Philippines,                   // PH
    Pakistan,                      // PK
    Poland,                        // PL
    SaintPierreAndMiquelon,        // PM
    Pitcairn,                      // PN
    PuertoRico,                    // PR
    PalestinianTerritory,          // PS
    Portugal,                      // PT
    Palau,                         // PW
    Paraguay,                      // PY
    Qatar,                         // QA
    Reunion,                       // RE
    Romania,                       // RO
    Russia,                        // RU
    Rwanda,                        // RW
    SaudiArabia,                   // SA
    SolomonIslands,                // SB
    Seychelles,                    // SC
    Sudan,                         // SD
    Sweden,                        // SE
    Singapore,                     // SG
    SaintHelena,                   // SH
    Slovenia,                      // SI
    SvalbardAndJanMayen,           // SJ
    Slovakia,                      // SK
    SierraLeone,                   // SL
    SanMarino,                     // SM
    Senegal,                       // SN
    Somalia,                       // SO
    Suriname,                      // SR
    SaoTomeAndPrincipe,            // ST
    ElSalvador,                    // SV
    SyrianArabRepublic,            // SY
    Swaziland,                     // SZ
    TurksAndCaicosIslands,         // TC
    Chad,                          // TD
    FrenchSouthernTerritories,     // TF
    Togo,                          // TG
    Thailand,                      // TH
    Tajikistan,                    // TJ
    Tokelau,                       // TK
    Turkmenistan,                  // TM
    Tunisia,                       // TN
    Tonga,                         // TO
    TimorLeste,                    // TL
    Turkey,                        // TR
    TrinidadAndTobago,             // TT
    Tuvalu,                        // TV
    Taiwan,                        // TW
    Tanzania,                      // TZ
    Ukraine,                       // UA
    Uganda,                        // UG
    USMinorOutlyingIslands,        // UM
    UnitedStates,                  // US
    Uruguay,                       // UY
    Uzbekistan,                    // UZ
    Vatican,                       // VA
    SaintVincentAndTheGrenadines,  // VC
    Venezuela,                     // VE
    BritishVirginIslands,          // VG
    USVirginIslands,               // VI
    Vietnam,                       // VN
    Vanuatu,                       // VU
    WallisAndFutuna,               // WF
    Samoa,                         // WS
    Yemen,                         // YE
    Mayotte,                       // YT
    Serbia,                        // RS
    SouthAfrica,                   // ZA
    Zambia,                        // ZM
    Montenegro,                    // ME
    Zimbabwe,                      // ZW
    Unknown2,                      // XX
    SatelliteProvider,             // A2
    OtherCountry,                  // O1
    AlandIslands,                  // AX
    Guernsey,                      // GG
    IsleOfMan,                     // IM
    Jersey,                        // JE
    SaintBarthelemy,               // BL
    SaintMartin,                   // MF
}

impl Country {
    pub fn from_iso3166_2_bytes(bytes: [u8; 2]) -> Country {
        Country::from_iso3166_2_u16(u16::from_le_bytes(bytes))
    }

    fn from_iso3166_2_u16(value: u16) -> Country {
        match value {
            0x5858 => Country::Unknown,                       // XX Unknown Country
            0x434f => Country::Oceania,                       // OC Oceania Continent
            0x5545 => Country::Europe,                        // EU Europe
            0x4441 => Country::Andorra,                       // AD Andorra
            0x4541 => Country::UAE,                           // AE United Arabian Emirates
            0x4641 => Country::Afghanistan,                   // AF Afghanistan
            0x4741 => Country::Antigua,                       // AG Antigua
            0x4941 => Country::Anguilla,                      // AI Anguilla
            0x4c41 => Country::Albania,                       // AL Albania
            0x4d41 => Country::Armenia,                       // AM Armenia
            0x4e41 => Country::Antilles,                      // AN Antilles
            0x4f41 => Country::Angola,                        // AO Angola
            0x5141 => Country::Antarctica,                    // AQ Antarctica
            0x5241 => Country::Argentina,                     // AR Argentina
            0x5341 => Country::AmericanSamoa,                 // AS American Samoa
            0x5441 => Country::Austria,                       // AT Austria
            0x5541 => Country::Australia,                     // AU Australia
            0x5741 => Country::Aruba,                         // AW Aruba
            0x5a41 => Country::Azerbaijan,                    // AZ Azerbaijan
            0x4142 => Country::Bosnia,                        // BA Bosnia
            0x4242 => Country::Barbados,                      // BB Barbados
            0x4442 => Country::Bangladesh,                    // BD Bangladesh
            0x4542 => Country::Belgium,                       // BE Belgium
            0x4642 => Country::BurkinaFaso,                   // BF Burkina Faso
            0x4742 => Country::Bulgaria,                      // BG Bulgaria
            0x4842 => Country::Bahrain,                       // BH Bahrain
            0x4942 => Country::Burundi,                       // BI Burundi
            0x4a42 => Country::Benin,                         // BJ Benin
            0x4d42 => Country::Bermuda,                       // BM Bermuda
            0x4e42 => Country::BruneiDarussalam,              // BN Brunei Darussalam
            0x4f42 => Country::Bolivia,                       // BO Bolivia
            0x5242 => Country::Brazil,                        // BR Brazil
            0x5342 => Country::Bahamas,                       // BS Bahamas
            0x5442 => Country::Bhutan,                        // BT Bhutan
            0x5642 => Country::BouvetIsland,                  // BV Bouvet Island
            0x5742 => Country::Botswana,                      // BW Botswana
            0x5942 => Country::Belarus,                       // BY Belarus
            0x5a42 => Country::Belize,                        // BZ Belize
            0x4143 => Country::Canada,                        // CA Canada
            0x4343 => Country::CocosIslands,                  // CC Cocos (Keeling) Islands
            0x4443 => Country::DRCongo,                       // CD Congo, The Democratic Republic of them (still only Congo to osu!)
            0x4643 => Country::CentralAfricanRepublic,        // CF Central African Republic
            0x4743 => Country::Congo,                         // CG Congo
            0x4843 => Country::Switzerland,                   // CH Switzerland
            0x4943 => Country::CoteDIvoire,                   // CI Cote d'Ivoire
            0x4b43 => Country::CookIslands,                   // CK Cook Islands
            0x4c43 => Country::Chile,                         // CL Chile
            0x4d43 => Country::Cameroon,                      // CM Cameroon
            0x4e43 => Country::China,                         // CN China
            0x4f43 => Country::Colombia,                      // CO Colombia
            0x5243 => Country::CostaRica,                     // CR Costa Rica
            0x5543 => Country::Cuba,                          // CU Cuba
            0x5643 => Country::CapeVerde,                     // CV Cape Verde
            0x5843 => Country::ChristmasIsland,               // CX Christmas Island
            0x5943 => Country::Cyprus,                        // CY Cyprus
            0x5a43 => Country::CzechRepublic,                 // CZ Czech Republic
            0x4544 => Country::Germany,                       // DE Germany
            0x4a44 => Country::Djibouti,                      // DJ Djibouti
            0x4b44 => Country::Denmark,                       // DK Denmark
            0x4d44 => Country::Dominica,                      // DM Dominica
            0x4f44 => Country::DominicanRepublic,             // DO Dominican Republic
            0x5a44 => Country::Algeria,                       // DZ Algeria
            0x4345 => Country::Ecuador,                       // EC Ecuador
            0x4545 => Country::Estonia,                       // EE Estonia
            0x4745 => Country::Egypt,                         // EG Egypt
            0x4845 => Country::WesternSahara,                 // EH Western Sahara
            0x5245 => Country::Eritrea,                       // ER Eritrea
            0x5345 => Country::Spain,                         // ES Spain
            0x5445 => Country::Ethiopia,                      // ET Ethiopia
            0x4946 => Country::Finland,                       // FI Finland
            0x4a46 => Country::Fiji,                          // FJ Fiji
            0x4b46 => Country::FalklandIslands,               // FK Falkland Islands (Malvinas)
            0x4d46 => Country::Micronesia,                    // FM Micronesia, Federated States of
            0x4f46 => Country::FaroeIslands,                  // FO Faroe Islands
            0x5246 => Country::France,                        // FR France
            0x5846 => Country::MetropolitanFrance,            // FX Metropolitan France
            0x4147 => Country::Gabon,                         // GA Gabon
            0x4247 => Country::UnitedKingdom,                 // GB United Kingdom
            0x4447 => Country::Grenada,                       // GD Grenada
            0x4547 => Country::Georgia,                       // GE Georgia
            0x4647 => Country::FrenchGuiana,                  // GF French Guiana
            0x4847 => Country::Ghana,                         // GH Ghana
            0x4947 => Country::Gibraltar,                     // GI Gibraltar
            0x4c47 => Country::Greenland,                     // GL Greenland
            0x4d47 => Country::Gambia,                        // GM Gambia
            0x4e47 => Country::Guinea,                        // GN Guinea
            0x5047 => Country::Guadeloupe,                    // GP Guadeloupe
            0x5147 => Country::EquatorialGuinea,              // GQ Equatorial Guinea
            0x5247 => Country::Greece,                        // GR Greece
            0x5347 => Country::SouthGeorgia,                  // GS South Georgia and the South Sandwich Islands
            0x5447 => Country::Guatemala,                     // GT Guatemala
            0x5547 => Country::Guam,                          // GU Guam
            0x5747 => Country::GuineaBissau,                  // GW Guinea-Bissau
            0x5947 => Country::Guyana,                        // GY Guyana
            0x4b48 => Country::HongKong,                      // HK Hong Kong
            0x4d48 => Country::HeardIslandAndMcDonaldIslands, // HM Heard Island and McDonald Islands
            0x4e48 => Country::Honduras,                      // HN Honduras
            0x5248 => Country::Croatia,                       // HR Croatia
            0x5448 => Country::Haiti,                         // HT Haiti
            0x5548 => Country::Hungary,                       // HU Hungary
            0x4449 => Country::Indonesia,                     // ID Indonesia
            0x4549 => Country::Ireland,                       // IE Ireland
            0x4c49 => Country::Israel,                        // IL Israel
            0x4e49 => Country::India,                         // IN India
            0x4f49 => Country::BritishIndianOceanTerritory,   // IO British Indian Ocean Territory
            0x5149 => Country::Iraq,                          // IQ Iraq
            0x5249 => Country::Iran,                          // IR Iran, Islamic Republic of
            0x5349 => Country::Iceland,                       // IS Iceland
            0x5449 => Country::Italy,                         // IT Italy
            0x4d4a => Country::Jamaica,                       // JM Jamaica
            0x4f4a => Country::Jordan,                        // JO Jordan
            0x504a => Country::Japan,                         // JP Japan
            0x454b => Country::Kenya,                         // KE Kenya
            0x474b => Country::Kyrgyzstan,                    // KG Kyrgyzstan
            0x484b => Country::Cambodia,                      // KH Cambodia
            0x494b => Country::Kiribati,                      // KI Kiribati
            0x4d4b => Country::Comoros,                       // KM Comoros
            0x4e4b => Country::SaintKittsAndNevis,            // KN Saint Kitts and Nevis
            0x504b => Country::NorthKorea,                    // KP Korea, Democratic People's Republic of
            0x524b => Country::SouthKorea,                    // KR Korea, Republic of
            0x574b => Country::Kuwait,                        // KW Kuwait
            0x594b => Country::CaymanIslands,                 // KY Cayman Islands
            0x5a4b => Country::Kazakhstan,                    // KZ Kazakhstan
            0x414c => Country::Laos,                          // LA Lao People's Democratic Republic
            0x424c => Country::Lebanon,                       // LB Lebanon
            0x434c => Country::SaintLucia,                    // LC Saint Lucia
            0x494c => Country::Liechtenstein,                 // LI Liechtenstein
            0x4b4c => Country::SriLanka,                      // LK Sri Lanka
            0x524c => Country::Liberia,                       // LR Liberia
            0x534c => Country::Lesotho,                       // LS Lesotho
            0x544c => Country::Lithuania,                     // LT Lithuania
            0x554c => Country::Luxembourg,                    // LU Luxembourg
            0x564c => Country::Latvia,                        // LV Latvia
            0x594c => Country::Libya,                         // LY Libyan Arab Jamahiriya
            0x414d => Country::Morocco,                       // MA Morocco
            0x434d => Country::Monaco,                        // MC Monaco
            0x444d => Country::Moldova,                       // MD Moldova, Republic of
            0x474d => Country::Madagascar,                    // MG Madagascar
            0x484d => Country::MarshallIslands,               // MH Marshall Islands
            0x4b4d => Country::Macedonia,                     // MK Macedonia
            0x4c4d => Country::Mali,                          // ML Mali
            0x4d4d => Country::Myanmar,                       // MM Myanmar
            0x4e4d => Country::Mongolia,                      // MN Mongolia
            0x4f4d => Country::Macao,                         // MO Macao
            0x504d => Country::NorthernMarianaIslands,        // MP Northern Mariana Islands
            0x514d => Country::Martinique,                    // MQ Martinique
            0x524d => Country::Mauritania,                    // MR Mauritania
            0x534d => Country::Montserrat,                    // MS Montserrat
            0x544d => Country::Malta,                         // MT Malta
            0x554d => Country::Mauritius,                     // MU Mauritius
            0x564d => Country::Maldives,                      // MV Maldives
            0x574d => Country::Malawi,                        // MW Malawi
            0x584d => Country::Mexico,                        // MX Mexico
            0x594d => Country::Malaysia,                      // MY Malaysia
            0x5a4d => Country::Mozambique,                    // MZ Mozambique
            0x414e => Country::Namibia,                       // NA Namibia
            0x434e => Country::NewCaledonia,                  // NC New Caledonia
            0x454e => Country::Niger,                         // NE Niger
            0x464e => Country::NorfolkIsland,                 // NF Norfolk Island
            0x474e => Country::Nigeria,                       // NG Nigeria
            0x494e => Country::Nicaragua,                     // NI Nicaragua
            0x4c4e => Country::Netherlands,                   // NL Netherlands
            0x4f4e => Country::Norway,                        // NO Norway
            0x504e => Country::Nepal,                         // NP Nepal
            0x524e => Country::Nauru,                         // NR Nauru
            0x554e => Country::Niue,                          // NU Niue
            0x5a4e => Country::NewZealand,                    // NZ New Zealand
            0x4d4f => Country::Oman,                          // OM Oman
            0x4150 => Country::Panama,                        // PA Panama
            0x4550 => Country::Peru,                          // PE Peru
            0x4650 => Country::FrenchPolynesia,               // PF French Polynesia
            0x4750 => Country::PapuaNewGuinea,                // PG Papua New Guinea
            0x4850 => Country::Philippines,                   // PH Philippines
            0x4b50 => Country::Pakistan,                      // PK Pakistan
            0x4c50 => Country::Poland,                        // PL Poland
            0x4d50 => Country::SaintPierreAndMiquelon,        // PM Saint Pierre and Miquelon
            0x4e50 => Country::Pitcairn,                      // PN Pitcairn
            0x5250 => Country::PuertoRico,                    // PR Puerto Rico
            0x5350 => Country::PalestinianTerritory,          // PS Palestinian Territory
            0x5450 => Country::Portugal,                      // PT Portugal
            0x5750 => Country::Palau,                         // PW Palau
            0x5950 => Country::Paraguay,                      // PY Paraguay
            0x4151 => Country::Qatar,                         // QA Qatar
            0x4552 => Country::Reunion,                       // RE Reunion
            0x4f52 => Country::Romania,                       // RO Romania
            0x5552 => Country::Russia,                        // RU Russian Federation
            0x5752 => Country::Rwanda,                        // RW Rwanda
            0x4153 => Country::SaudiArabia,                   // SA Saudi Arabia
            0x4253 => Country::SolomonIslands,                // SB Solomon Islands
            0x4353 => Country::Seychelles,                    // SC Seychelles
            0x4453 => Country::Sudan,                         // SD Sudan
            0x4553 => Country::Sweden,                        // SE Sweden
            0x4753 => Country::Singapore,                     // SG Singapore
            0x4853 => Country::SaintHelena,                   // SH Saint Helena
            0x4953 => Country::Slovenia,                      // SI Slovenia
            0x4a53 => Country::SvalbardAndJanMayen,           // SJ Svalbard and Jan Mayen
            0x4b53 => Country::Slovakia,                      // SK Slovakia
            0x4c53 => Country::SierraLeone,                   // SL Sierra Leone
            0x4d53 => Country::SanMarino,                     // SM San Marino
            0x4e53 => Country::Senegal,                       // SN Senegal
            0x4f53 => Country::Somalia,                       // SO Somalia
            0x5253 => Country::Suriname,                      // SR Suriname
            0x5453 => Country::SaoTomeAndPrincipe,            // ST Sao Tome and Principe
            0x5653 => Country::ElSalvador,                    // SV El Salvador
            0x5953 => Country::SyrianArabRepublic,            // SY Syrian Arab Republic
            0x5a53 => Country::Swaziland,                     // SZ Swaziland
            0x4354 => Country::TurksAndCaicosIslands,         // TC Turks and Caicos Islands
            0x4454 => Country::Chad,                          // TD Chad
            0x4654 => Country::FrenchSouthernTerritories,     // TF French Southern Territories
            0x4754 => Country::Togo,                          // TG Togo
            0x4854 => Country::Thailand,                      // TH Thailand
            0x4a54 => Country::Tajikistan,                    // TJ Tajikistan
            0x4b54 => Country::Tokelau,                       // TK Tokelau
            0x4d54 => Country::Turkmenistan,                  // TM Turkmenistan
            0x4e54 => Country::Tunisia,                       // TN Tunisia
            0x4f54 => Country::Tonga,                         // TO Tonga
            0x4c54 => Country::TimorLeste,                    // TL Timor-Leste
            0x5254 => Country::Turkey,                        // TR Turkey
            0x5454 => Country::TrinidadAndTobago,             // TT Trinidad and Tobago
            0x5654 => Country::Tuvalu,                        // TV Tuvalu
            0x5754 => Country::Taiwan,                        // TW Taiwan
            0x5a54 => Country::Tanzania,                      // TZ Tanzania, United Republic of
            0x4155 => Country::Ukraine,                       // UA Ukraine
            0x4755 => Country::Uganda,                        // UG Uganda
            0x4d55 => Country::USMinorOutlyingIslands,        // UM United States Minor Outlying Islands
            0x5355 => Country::UnitedStates,                  // US United States
            0x5955 => Country::Uruguay,                       // UY Uruguay
            0x5a55 => Country::Uzbekistan,                    // UZ Uzbekistan
            0x4156 => Country::Vatican,                       // VA Holy See (Vatican City State)
            0x4356 => Country::SaintVincentAndTheGrenadines,  // VC Saint Vincent and the Grenadines
            0x4556 => Country::Venezuela,                     // VE Venezuela
            0x4756 => Country::BritishVirginIslands,          // VG Virgin Islands, British
            0x4956 => Country::USVirginIslands,               // VI Virgin Islands, U.S.
            0x4e56 => Country::Vietnam,                       // VN Vietnam
            0x5556 => Country::Vanuatu,                       // VU Vanuatu
            0x4657 => Country::WallisAndFutuna,               // WF Wallis and Futuna
            0x5357 => Country::Samoa,                         // WS Samoa
            0x4559 => Country::Yemen,                         // YE Yemen
            0x5459 => Country::Mayotte,                       // YT Mayotte
            0x5352 => Country::Serbia,                        // RS Serbia
            0x415a => Country::SouthAfrica,                   // ZA South Africa
            0x4d5a => Country::Zambia,                        // ZM Zambia
            0x454d => Country::Montenegro,                    // ME Montenegro
            0x575a => Country::Zimbabwe,                      // ZW Zimbabwe
            0x3241 => Country::SatelliteProvider,             // A2 Satellite Provider
            0x314f => Country::OtherCountry,                  // O1 Other Country
            0x5841 => Country::AlandIslands,                  // AX Aland Islands
            0x4747 => Country::Guernsey,                      // GG Guernsey
            0x4d49 => Country::IsleOfMan,                     // IM Isle of Man
            0x454a => Country::Jersey,                        // JE Jersey
            0x4c42 => Country::SaintBarthelemy,               // BL Saint Barthelemy
            0x464d => Country::SaintMartin,                   // MF Saint Martin
            _ => Country::Unknown2,
        }
    }

    pub unsafe fn from_iso3166_2_unchecked(country_code: &str) -> Country {
        Country::from_iso3166_2_u16(*(country_code.as_ptr() as *const u16))
    }

    pub fn try_from_iso3166_2(country_code: &str) -> Result<Country> {
        if country_code.len() != 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid country code length",
            ));
        }

        let country_code = country_code.as_bytes();
        let country_code = country_code.first_chunk().expect("???");
        Ok(Country::from_iso3166_2_u16(u16::from_le_bytes(
            *country_code,
        )))
    }

    pub const fn code(&self) -> &'static str {
        match self {
            Country::Unknown => "XX",
            Country::Oceania => "OC",
            Country::Europe => "EU",
            Country::Andorra => "AD",
            Country::UAE => "AE",
            Country::Afghanistan => "AF",
            Country::Antigua => "AG",
            Country::Anguilla => "AI",
            Country::Albania => "AL",
            Country::Armenia => "AM",
            Country::Antilles => "AN",
            Country::Angola => "AO",
            Country::Antarctica => "AQ",
            Country::Argentina => "AR",
            Country::AmericanSamoa => "AS",
            Country::Austria => "AT",
            Country::Australia => "AU",
            Country::Aruba => "AW",
            Country::Azerbaijan => "AZ",
            Country::Bosnia => "BA",
            Country::Barbados => "BB",
            Country::Bangladesh => "BD",
            Country::Belgium => "BE",
            Country::BurkinaFaso => "BF",
            Country::Bulgaria => "BG",
            Country::Bahrain => "BH",
            Country::Burundi => "BI",
            Country::Benin => "BJ",
            Country::Bermuda => "BM",
            Country::BruneiDarussalam => "BN",
            Country::Bolivia => "BO",
            Country::Brazil => "BR",
            Country::Bahamas => "BS",
            Country::Bhutan => "BT",
            Country::BouvetIsland => "BV",
            Country::Botswana => "BW",
            Country::Belarus => "BY",
            Country::Belize => "BZ",
            Country::Canada => "CA",
            Country::CocosIslands => "CC",
            Country::DRCongo => "CD",
            Country::CentralAfricanRepublic => "CF",
            Country::Congo => "CG",
            Country::Switzerland => "CH",
            Country::CoteDIvoire => "CI",
            Country::CookIslands => "CK",
            Country::Chile => "CL",
            Country::Cameroon => "CM",
            Country::China => "CN",
            Country::Colombia => "CO",
            Country::CostaRica => "CR",
            Country::Cuba => "CU",
            Country::CapeVerde => "CV",
            Country::ChristmasIsland => "CX",
            Country::Cyprus => "CY",
            Country::CzechRepublic => "CZ",
            Country::Germany => "DE",
            Country::Djibouti => "DJ",
            Country::Denmark => "DK",
            Country::Dominica => "DM",
            Country::DominicanRepublic => "DO",
            Country::Algeria => "DZ",
            Country::Ecuador => "EC",
            Country::Estonia => "EE",
            Country::Egypt => "EG",
            Country::WesternSahara => "EH",
            Country::Eritrea => "ER",
            Country::Spain => "ES",
            Country::Ethiopia => "ET",
            Country::Finland => "FI",
            Country::Fiji => "FJ",
            Country::FalklandIslands => "FK",
            Country::Micronesia => "FM",
            Country::FaroeIslands => "FO",
            Country::France => "FR",
            Country::MetropolitanFrance => "FX",
            Country::Gabon => "GA",
            Country::UnitedKingdom => "GB",
            Country::Grenada => "GD",
            Country::Georgia => "GE",
            Country::FrenchGuiana => "GF",
            Country::Ghana => "GH",
            Country::Gibraltar => "GI",
            Country::Greenland => "GL",
            Country::Gambia => "GM",
            Country::Guinea => "GN",
            Country::Guadeloupe => "GP",
            Country::EquatorialGuinea => "GQ",
            Country::Greece => "GR",
            Country::SouthGeorgia => "GS",
            Country::Guatemala => "GT",
            Country::Guam => "GU",
            Country::GuineaBissau => "GW",
            Country::Guyana => "GY",
            Country::HongKong => "HK",
            Country::HeardIslandAndMcDonaldIslands => "HM",
            Country::Honduras => "HN",
            Country::Croatia => "HR",
            Country::Haiti => "HT",
            Country::Hungary => "HU",
            Country::Indonesia => "ID",
            Country::Ireland => "IE",
            Country::Israel => "IL",
            Country::India => "IN",
            Country::BritishIndianOceanTerritory => "IO",
            Country::Iraq => "IQ",
            Country::Iran => "IR",
            Country::Iceland => "IS",
            Country::Italy => "IT",
            Country::Jamaica => "JM",
            Country::Jordan => "JO",
            Country::Japan => "JP",
            Country::Kenya => "KE",
            Country::Kyrgyzstan => "KG",
            Country::Cambodia => "KH",
            Country::Kiribati => "KI",
            Country::Comoros => "KM",
            Country::SaintKittsAndNevis => "KN",
            Country::NorthKorea => "KP",
            Country::SouthKorea => "KR",
            Country::Kuwait => "KW",
            Country::CaymanIslands => "KY",
            Country::Kazakhstan => "KZ",
            Country::Laos => "LA",
            Country::Lebanon => "LB",
            Country::SaintLucia => "LC",
            Country::Liechtenstein => "LI",
            Country::SriLanka => "LK",
            Country::Liberia => "LR",
            Country::Lesotho => "LS",
            Country::Lithuania => "LT",
            Country::Luxembourg => "LU",
            Country::Latvia => "LV",
            Country::Libya => "LY",
            Country::Morocco => "MA",
            Country::Monaco => "MC",
            Country::Moldova => "MD",
            Country::Madagascar => "MG",
            Country::MarshallIslands => "MH",
            Country::Macedonia => "MK",
            Country::Mali => "ML",
            Country::Myanmar => "MM",
            Country::Mongolia => "MN",
            Country::Macao => "MO",
            Country::NorthernMarianaIslands => "MP",
            Country::Martinique => "MQ",
            Country::Mauritania => "MR",
            Country::Montserrat => "MS",
            Country::Malta => "MT",
            Country::Mauritius => "MU",
            Country::Maldives => "MV",
            Country::Malawi => "MW",
            Country::Mexico => "MX",
            Country::Malaysia => "MY",
            Country::Mozambique => "MZ",
            Country::Namibia => "NA",
            Country::NewCaledonia => "NC",
            Country::Niger => "NE",
            Country::NorfolkIsland => "NF",
            Country::Nigeria => "NG",
            Country::Nicaragua => "NI",
            Country::Netherlands => "NL",
            Country::Norway => "NO",
            Country::Nepal => "NP",
            Country::Nauru => "NR",
            Country::Niue => "NU",
            Country::NewZealand => "NZ",
            Country::Oman => "OM",
            Country::Panama => "PA",
            Country::Peru => "PE",
            Country::FrenchPolynesia => "PF",
            Country::PapuaNewGuinea => "PG",
            Country::Philippines => "PH",
            Country::Pakistan => "PK",
            Country::Poland => "PL",
            Country::SaintPierreAndMiquelon => "PM",
            Country::Pitcairn => "PN",
            Country::PuertoRico => "PR",
            Country::PalestinianTerritory => "PS",
            Country::Portugal => "PT",
            Country::Palau => "PW",
            Country::Paraguay => "PY",
            Country::Qatar => "QA",
            Country::Reunion => "RE",
            Country::Romania => "RO",
            Country::Russia => "RU",
            Country::Rwanda => "RW",
            Country::SaudiArabia => "SA",
            Country::SolomonIslands => "SB",
            Country::Seychelles => "SC",
            Country::Sudan => "SD",
            Country::Sweden => "SE",
            Country::Singapore => "SG",
            Country::SaintHelena => "SH",
            Country::Slovenia => "SI",
            Country::SvalbardAndJanMayen => "SJ",
            Country::Slovakia => "SK",
            Country::SierraLeone => "SL",
            Country::SanMarino => "SM",
            Country::Senegal => "SN",
            Country::Somalia => "SO",
            Country::Suriname => "SR",
            Country::SaoTomeAndPrincipe => "ST",
            Country::ElSalvador => "SV",
            Country::SyrianArabRepublic => "SY",
            Country::Swaziland => "SZ",
            Country::TurksAndCaicosIslands => "TC",
            Country::Chad => "TD",
            Country::FrenchSouthernTerritories => "TF",
            Country::Togo => "TG",
            Country::Thailand => "TH",
            Country::Tajikistan => "TJ",
            Country::Tokelau => "TK",
            Country::Turkmenistan => "TM",
            Country::Tunisia => "TN",
            Country::Tonga => "TO",
            Country::TimorLeste => "TL",
            Country::Turkey => "TR",
            Country::TrinidadAndTobago => "TT",
            Country::Tuvalu => "TV",
            Country::Taiwan => "TW",
            Country::Tanzania => "TZ",
            Country::Ukraine => "UA",
            Country::Uganda => "UG",
            Country::USMinorOutlyingIslands => "UM",
            Country::UnitedStates => "US",
            Country::Uruguay => "UY",
            Country::Uzbekistan => "UZ",
            Country::Vatican => "VA",
            Country::SaintVincentAndTheGrenadines => "VC",
            Country::Venezuela => "VE",
            Country::BritishVirginIslands => "VG",
            Country::USVirginIslands => "VI",
            Country::Vietnam => "VN",
            Country::Vanuatu => "VU",
            Country::WallisAndFutuna => "WF",
            Country::Samoa => "WS",
            Country::Yemen => "YE",
            Country::Mayotte => "YT",
            Country::Serbia => "RS",
            Country::SouthAfrica => "ZA",
            Country::Zambia => "ZM",
            Country::Montenegro => "ME",
            Country::Zimbabwe => "ZW",
            Country::Unknown2 => "XX",
            Country::SatelliteProvider => "A2",
            Country::OtherCountry => "O1",
            Country::AlandIslands => "AX",
            Country::Guernsey => "GG",
            Country::IsleOfMan => "IM",
            Country::Jersey => "JE",
            Country::SaintBarthelemy => "BL",
            Country::SaintMartin => "MF",
        }
    }
}
