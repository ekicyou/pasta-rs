# SHIORI/3.0 sample


=====send=====
(load library)
=====send=====
(function : load found)
=====send=====
(function : request found)
=====send=====
(function : unload found)
=====send=====
(getversion/loadex call)
=====send=====
(load call)
=====send=====
GET Version SHIORI/2.6
Charset: UTF-8
Sender: SSP


=====response=====
SHIORI/3.0 200 OK
ID: Satori
Craftman: Yagi Kushigahama/The Maintenance Shop
Version: phase Mc159-2
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
ID: version
SecurityLevel: local
Sender: SSP


=====response=====
SHIORI/3.0 200 OK
Value: phase Mc159-2
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnInitialize
Reference0: 


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: ownerghostname
Reference0: かなで


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: basewareversion
Reference0: 2.3.86
Reference1: SSP
Reference2: 2.3.86.3000


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: capability
Reference0: request.status
Reference1: request.securitylevel
Reference2: request.baseid
Reference3: response.marker
Reference4: response.errorlevel
Reference5: response.errordescription
Reference6: response.securitylevel
Reference7: response.requestcharset


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnNotifyOSInfo
Reference0: WindowsNT,10.0,Windows 10 RSx
Reference1: Intel,3500,0.6.12.3,bfcbfbff
Reference2: 16710640,2097024
Reference3: 64


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnNotifyFontInfo
Reference0: System
Reference1: Terminal
Reference2: FixedSys
Reference3: Modern
Reference4: Roman
Reference5: Script
Reference6: Courier
Reference7: MS Serif
Reference8: MS Sans Serif
Reference9: Small Fonts
Reference10: Terminal Greek 737 (437G)
Reference11: 8514oem
Reference12: DJS 秀英明朝 StdN M
Reference13: DJS 秀英横太明朝 StdN B
Reference14: DJS 秀英横太明朝 StdN M
Reference15: Terminal Greek 869
Reference16: DINPro-Black
Reference17: DINPro-Bold
Reference18: DINPro-Light
Reference19: DINPro-Medium
Reference20: DINPro-Regular
Reference21: HelveticaNeueLT Pro 55 Roman
Reference22: Trajan Pro
Reference23: Function Pro Book
Reference24: Function Pro Medium
Reference25: Marlett
Reference26: Arial
Reference27: Arial Black
Reference28: Calibri
Reference29: Calibri Light
Reference30: Cambria
Reference31: Cambria Math
Reference32: Candara
Reference33: Comic Sans MS
Reference34: Consolas
Reference35: Constantia
Reference36: Corbel
Reference37: Courier New
Reference38: Ebrima
Reference39: Franklin Gothic Medium
Reference40: Gabriola
Reference41: Gadugi
Reference42: Georgia
Reference43: HoloLens MDL2 Assets
Reference44: Impact
Reference45: Javanese Text
Reference46: Leelawadee UI
Reference47: Leelawadee UI Semilight
Reference48: Lucida Console
Reference49: Lucida Sans Unicode
Reference50: Malgun Gothic
Reference51: Malgun Gothic Semilight
Reference52: Microsoft Himalaya
Reference53: Microsoft JhengHei
Reference54: Microsoft JhengHei UI
Reference55: Microsoft JhengHei Light
Reference56: Microsoft JhengHei UI Light
Reference57: Microsoft New Tai Lue
Reference58: Microsoft PhagsPa
Reference59: Microsoft Sans Serif
Reference60: Microsoft Tai Le
Reference61: Microsoft YaHei
Reference62: Microsoft YaHei UI
Reference63: Microsoft YaHei Light
Reference64: Microsoft YaHei UI Light
Reference65: Microsoft Yi Baiti
Reference66: MingLiU-ExtB
Reference67: PMingLiU-ExtB
Reference68: MingLiU_HKSCS-ExtB
Reference69: Mongolian Baiti
Reference70: ＭＳ ゴシック
Reference71: MS UI Gothic
Reference72: ＭＳ Ｐゴシック
Reference73: MV Boli
Reference74: Myanmar Text
Reference75: Nirmala UI
Reference76: Nirmala UI Semilight
Reference77: Palatino Linotype
Reference78: Segoe MDL2 Assets
Reference79: Segoe Print
Reference80: Segoe Script
Reference81: Segoe UI
Reference82: Segoe UI Black
Reference83: Segoe UI Emoji
Reference84: Segoe UI Historic
Reference85: Segoe UI Light
Reference86: Segoe UI Semibold
Reference87: Segoe UI Semilight
Reference88: Segoe UI Symbol
Reference89: SimSun
Reference90: NSimSun
Reference91: SimSun-ExtB
Reference92: Sitka Small
Reference93: Sitka Text
Reference94: Sitka Subheading
Reference95: Sitka Heading
Reference96: Sitka Display
Reference97: Sitka Banner
Reference98: Sylfaen
Reference99: Symbol
Reference100: Tahoma
Reference101: Times New Roman
Reference102: Trebuchet MS
Reference103: Verdana
Reference104: Webdings
Reference105: Wingdings
Reference106: 游ゴシック
Reference107: Yu Gothic UI
Reference108: Yu Gothic UI Semibold
Reference109: 游ゴシック Light
Reference110: Yu Gothic UI Light
Reference111: 游ゴシック Medium
Reference112: Yu Gothic UI Semilight
Reference113: メイリオ
Reference114: Meiryo UI
Reference115: ＭＳ 明朝
Reference116: ＭＳ Ｐ明朝
Reference117: 游明朝
Reference118: 游明朝 Demibold
Reference119: 游明朝 Light
Reference120: Razer Text Regular
Reference121: Razer Header Light
Reference122: Razer Header Regular
Reference123: Razer Header Regular Oblique
Reference124: Agency FB
Reference125: Algerian
Reference126: Book Antiqua
Reference127: Arial Unicode MS
Reference128: Arial Rounded MT Bold
Reference129: 有澤太楷書
Reference130: 有澤太楷書P
Reference131: Baskerville Old Face
Reference132: Bauhaus 93
Reference133: Bell MT
Reference134: Bernard MT Condensed
Reference135: 恋文ペン字
Reference136: 麗流隷書
Reference137: Bodoni MT
Reference138: Bodoni MT Condensed
Reference139: Bodoni MT Black
Reference140: Bodoni MT Poster Compressed
Reference141: Bookman Old Style
Reference142: Bradley Hand ITC
Reference143: Britannic Bold
Reference144: Berlin Sans FB
Reference145: Berlin Sans FB Demi
Reference146: Broadway
Reference147: Brush Script MT
Reference148: Californian FB
Reference149: Calisto MT
Reference150: Castellar
Reference151: Century Schoolbook
Reference152: Centaur
Reference153: Century
Reference154: Chiller
Reference155: CHRISTINA
Reference156: Colonna MT
Reference157: Cooper Black
Reference158: Copperplate Gothic Bold
Reference159: Copperplate Gothic Light
Reference160: Curlz MT
Reference161: ＤＦ特太ゴシック体
Reference162: ＤＨＰ特太ゴシック体
Reference163: ＤＦ行書体
Reference164: ＤＨＰ行書体
Reference165: ＤＦ平成ゴシック体W5
Reference166: ＤＨＰ平成ゴシックW5
Reference167: ＤＦ平成明朝体W3
Reference168: ＤＨＰ平成明朝体W3
Reference169: ＤＦ平成明朝体W7
Reference170: ＤＨＰ平成明朝体W7
Reference171: 江戸勘亭流
Reference172: 江戸勘亭流Ｐ
Reference173: Elephant
Reference174: Engravers MT
Reference175: Eras Bold ITC
Reference176: Eras Demi ITC
Reference177: Eras Light ITC
Reference178: Eras Medium ITC
Reference179: 有澤行書
Reference180: 有澤楷書
Reference181: Felix Titling
Reference182: ふみゴシック
Reference183: 魚石行書
Reference184: 祥南行書体
Reference185: 祥南行書体P
Reference186: 正調祥南行書体
Reference187: 正調祥南行書体P
Reference188: FGW FONT
Reference189: Forte
Reference190: Franklin Gothic Book
Reference191: Franklin Gothic Demi
Reference192: Franklin Gothic Demi Cond
Reference193: Franklin Gothic Heavy
Reference194: Franklin Gothic Medium Cond
Reference195: Freestyle Script
Reference196: French Script MT
Reference197: Footlight MT Light
Reference198: 富士ポップ
Reference199: 富士ポップＰ
Reference200: Garamond
Reference201: Gigi
Reference202: Gill Sans MT
Reference203: Gill Sans MT Condensed
Reference204: Gill Sans Ultra Bold Condensed
Reference205: Gill Sans Ultra Bold
Reference206: Gloucester MT Extra Condensed
Reference207: Gill Sans MT Ext Condensed Bold
Reference208: Century Gothic
Reference209: Goudy Old Style
Reference210: Goudy Stout
Reference211: Harlow Solid Italic
Reference212: Harrington
Reference213: Haettenschweiler
Reference214: HGｺﾞｼｯｸE
Reference215: HGPｺﾞｼｯｸE
Reference216: HGSｺﾞｼｯｸE
Reference217: HGｺﾞｼｯｸM
Reference218: HGPｺﾞｼｯｸM
Reference219: HGSｺﾞｼｯｸM
Reference220: HG行書体
Reference221: HGP行書体
Reference222: HGS行書体
Reference223: HG教科書体
Reference224: HGP教科書体
Reference225: HGS教科書体
Reference226: HG明朝B
Reference227: HGP明朝B
Reference228: HGS明朝B
Reference229: HG明朝E
Reference230: HGP明朝E
Reference231: HGS明朝E
Reference232: HG創英角ﾎﾟｯﾌﾟ体
Reference233: HGP創英角ﾎﾟｯﾌﾟ体
Reference234: HGS創英角ﾎﾟｯﾌﾟ体
Reference235: HG創英ﾌﾟﾚｾﾞﾝｽEB
Reference236: HGP創英ﾌﾟﾚｾﾞﾝｽEB
Reference237: HGS創英ﾌﾟﾚｾﾞﾝｽEB
Reference238: HG創英角ｺﾞｼｯｸUB
Reference239: HGP創英角ｺﾞｼｯｸUB
Reference240: HGS創英角ｺﾞｼｯｸUB
Reference241: HG正楷書体-PRO
Reference242: HG丸ｺﾞｼｯｸM-PRO
Reference243: High Tower Text
Reference244: Imprint MT Shadow
Reference245: Informal Roman
Reference246: Blackadder ITC
Reference247: Edwardian Script ITC
Reference248: Kristen ITC
Reference249: CenturyOldst
Reference250: Embassy JS
Reference251: Fraktur JS
Reference252: %CenturyOldst
Reference253: Gothic720
Reference254: ARマーカー体E
Reference255: AR Pマーカー体E
Reference256: AR丸ゴシック体E
Reference257: AR P丸ゴシック体E
Reference258: AR丸ゴシック体M
Reference259: AR P丸ゴシック体M
Reference260: ARゴシック体M
Reference261: AR Pゴシック体M
Reference262: ARゴシック体S
Reference263: AR Pゴシック体S
Reference264: AR悠々ゴシック体E
Reference265: AR P悠々ゴシック体E
Reference266: ARマッチ体B
Reference267: AR Pマッチ体B
Reference268: &CenturyOldst
Reference269: &Gothic720
Reference270: AR楷書体M
Reference271: AR P楷書体M
Reference272: AR勘亭流H
Reference273: AR P勘亭流H
Reference274: AR明朝体L
Reference275: AR P明朝体L
Reference276: AR明朝体U
Reference277: AR P明朝体U
Reference278: Jokerman
Reference279: ARペン楷書体L
Reference280: AR Pペン楷書体L
Reference281: AR浪漫明朝体U
Reference282: AR P浪漫明朝体U
Reference283: JustEditMark
Reference284: JustHalfMarkG
Reference285: JustHalfMark
Reference286: ＪＳ平成明朝体W3
Reference287: JustKanaMarkG
Reference288: JustKanaMark
Reference289: JustOubunMarkG
Reference290: JustOubunMark
Reference291: $ＪＳゴシック
Reference292: $ＪＳ明朝
Reference293: JustUnitMarkG
Reference294: JustUnitMark
Reference295: JustWabunMarkG
Reference296: JustWabunMark
Reference297: AR教科書体M
Reference298: AR P教科書体M
Reference299: AR顏眞楷書体H
Reference300: AR P顏眞楷書体H
Reference301: Kunstler Script
Reference302: Wide Latin
Reference303: Lucida Bright
Reference304: Lucida Calligraphy
Reference305: Lucida Fax
Reference306: Lucida Handwriting
Reference307: Lucida Sans
Reference308: Lucida Sans Typewriter
Reference309: Magneto
Reference310: Maiandra GD
Reference311: Matura MT Script Capitals
Reference312: Mistral
Reference313: MoboGothic
Reference314: MoboExGothic
Reference315: Mobo90Gothic
Reference316: MoboEx90Gothic
Reference317: Modern No. 20
Reference318: MogaGothic
Reference319: MogaExGothic
Reference320: Moga90Gothic
Reference321: MogaEx90Gothic
Reference322: MogaMincho
Reference323: MogaExMincho
Reference324: Moga90Mincho
Reference325: MogaEx90Mincho
Reference326: Monotype Corsiva
Reference327: Myriad Web Pro
Reference328: Myriad Web Pro Condensed
Reference329: MYSTICAL
Reference330: Niagara Engraved
Reference331: Niagara Solid
Reference332: OCR A Extended
Reference333: OCRB
Reference334: Old English Text MT
Reference335: Onyx
Reference336: MS Outlook
Reference337: Palace Script MT
Reference338: Papyrus
Reference339: Parchment
Reference340: Perpetua
Reference341: Perpetua Titling MT
Reference342: Playbill
Reference343: Poor Richard
Reference344: Pristina
Reference345: Rage Italic
Reference346: Ravie
Reference347: Rockwell Condensed
Reference348: Rockwell
Reference349: Rockwell Extra Bold
Reference350: Script MT Bold
Reference351: Showcard Gothic
Reference352: Stencil
Reference353: 正調祥南行書体EX
Reference354: 正調祥南行書体EXP
Reference355: Tempus Sans ITC
Reference356: TGothic-GT01
Reference357: TPGothic-GT01
Reference358: TKaisho-GT01
Reference359: TPKaisho-GT01
Reference360: TMincho-GT01
Reference361: TPMincho-GT01
Reference362: Viner Hand ITC
Reference363: Vivaldi
Reference364: Vladimir Script
Reference365: Wingdings 2
Reference366: Wingdings 3
Reference367: YOzFontA
Reference368: YOzFontA90
Reference369: YOzFontAF
Reference370: YOzFontAF90
Reference371: YOzFontAP
Reference372: YOzFontAP90
Reference373: YOzFontC
Reference374: YOzFontC90
Reference375: YOzFontCF
Reference376: YOzFontCF90
Reference377: YOzFontE
Reference378: YOzFontEM
Reference379: YOzFontE90
Reference380: YOzFontEM90
Reference381: YOzFontEF
Reference382: YOzFontEMF
Reference383: YOzFontEF90
Reference384: YOzFontEMF90
Reference385: YOzFontN
Reference386: YOzFontNM
Reference387: YOzFontN90
Reference388: YOzFontNM90
Reference389: YOzFontNF
Reference390: YOzFontNMF
Reference391: YOzFontNF90
Reference392: YOzFontNMF90
Reference393: YOzFont
Reference394: YOzFont90
Reference395: YOzFontF
Reference396: YOzFontF90
Reference397: YOzFontP
Reference398: YOzFontP90
Reference399: YOzFontK
Reference400: YOzFontK90
Reference401: YOzFontKA
Reference402: YOzFontKA90
Reference403: YOzFontOTW
Reference404: YOzFontOTWL
Reference405: YOzFontOTWD
Reference406: YOzFontOTW Light
Reference407: YOzFontOTWL Light
Reference408: YOzFontOTWD Light
Reference409: YOzFontEX
Reference410: YOzFontEXM
Reference411: YOzFontEXF
Reference412: YOzFontEX90
Reference413: YOzFontEXM90
Reference414: YOzFontEXF90
Reference415: YOzFontNX
Reference416: YOzFontNXM
Reference417: YOzFontNXF
Reference418: YOzFontNX90
Reference419: YOzFontNXM90
Reference420: YOzFontNXF90
Reference421: YOzFontX
Reference422: YOzFontXM
Reference423: YOzFontXF
Reference424: YOzFontX90
Reference425: YOzFontXM90
Reference426: YOzFontXF90
Reference427: MT Extra
Reference428: Juice ITC
Reference429: Snap ITC
Reference430: Arial Narrow
Reference431: SWTOR Trajan
Reference432: SUSBalloon
Reference433: Buxton Sketch
Reference434: Segoe Marker
Reference435: SketchFlow Print
Reference436: DengXian
Reference437: Microsoft MHei
Reference438: Microsoft NeoGothic
Reference439: Yu Gothic
Reference440: Segoe WP Black
Reference441: Segoe WP
Reference442: Segoe WP Semibold
Reference443: Segoe WP Light
Reference444: Segoe WP SemiLight
Reference445: Dutch801 SWC
Reference446: Swiss742 SWC
Reference447: Bookshelf Symbol 7
Reference448: MS Reference Sans Serif
Reference449: MS Reference Specialty
Reference450: Tw Cen MT
Reference451: Tw Cen MT Condensed
Reference452: Tw Cen MT Condensed Extra Bold


=====response=====
***CallTime:2msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnNotifySelfInfo
Reference0: 奏でる日常の音色
Reference1: かなで
Reference2: メニューボード
Reference3: マスターシェル
Reference4: D:/wintools/ssp/ghost/kanade_2nd/shell/master/
Reference5: SSPデフォルト+
Reference6: D:/wintools/ssp/balloon/ssp/


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnNotifyBalloonInfo
Reference0: SSPデフォルト+
Reference1: D:/wintools/ssp/balloon/ssp/
Reference2: 0:0,1,2,3 1:0,1,2,3


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnNotifyShellInfo
Reference0: マスターシェル
Reference1: D:/wintools/ssp/ghost/kanade_2nd/shell/master/
Reference2: 0,1,2,3,4,5,6,7,8,9,10,20,21,22,23,24,25,26,27,28,29,30,31,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,1000,1001,1002,1003,1004,1005,1006,1007,1008,1009,1010,1011,1012,1013,1014,1015,1016,1017,1018,1019,1020,1021,1022,1023,1024,1025,1026,1027,1028,1100,1104,1105,1106,1107,1108,1109,1110,1111,1112,1113,1114,1115,1116,1117,1118,1119,1120,1121,1122,1123,1124,1125,1126,1127,1128,1129,1130,1131,1132,1133,1134,1135,1136,1137,1138,1139,1140,1200,1204,1205,1206,1207,1208,1209,1213,1214,1215,1216,1217,1218,1219,1220,1221,1222,1223,1224,1225,1226,1227,1228,1229,1230,1231,1232,1233,1234,1235,1236,1237,1238,1239,1240,1300,1304,1305,1306,1307,1308,1310,1313,1314,1315,1316,1317,1318,1319,1320,1321,1322,1323,1324,1325,1326,1327,1328,1329,1330,1331,1332,1333,1334,1335,1336,1337,1338,1339,1340,1400,1404,1405,1406,1407,1408,1411,1413,1414,1415,1416,1417,1418,1419,1420,1421,1422,1423,1424,1425,1426,1427,1428,1429,1430,1431,1432,1433,1434,1435,1436,1437,1438,1439,1440,1500,1502,1503,1504,1505,1506,1507,1508,1512,1513,1514,1515,1516,1517,1518,1519,1520,1521,1522,1523,1524,1525,1526,1527,1528,1529,1530,1531,1532,1533,1534,1535,1536,1537,1538,1539,1540,1600,1601,1603,1604,1605,1606,1607,1608,1613,1614,1615,1616,1617,1618,1619,1620,1621,1622,1623,1624,1625,1626,1627,1628,1629,1630,1631,1632,1633,1634,1635,1636,1637,1638,1639,1640


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: useorigin1


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: version


=====response=====
SHIORI/3.0 200 OK
Value: phase Mc159-2
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: name


=====response=====
SHIORI/3.0 200 OK
Value: Satori
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: craftman


=====response=====
SHIORI/3.0 200 OK
Value: Yagi Kushigahama/The Maintenance Shop
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: craftmanw


=====response=====
SHIORI/3.0 200 OK
Value: 櫛ヶ浜やぎ/整備班
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnNotifyUserInfo
Reference0: 正和
Reference1: 正和
Reference2: 
Reference3: undef


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
ID: otherghostname
SecurityLevel: local
Charset: UTF-8
Sender: SSP


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnNetworkStatusChange
Reference0: online
Reference1: 192.168.1.122001:0:9d38:6ab8:1c7b:2bb9:c540:e33f


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: ghostpathlist
Reference0: D:\wintools\ssp\ghost\
Reference1: D:\home\maz\git\edge-chan\mental-model\ghost\


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: balloonpathlist
Reference0: D:\wintools\ssp\balloon\
Reference1: D:\home\maz\git\edge-chan\mental-model\balloon\


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: headlinepathlist
Reference0: D:\wintools\ssp\headline\


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: pluginpathlist
Reference0: D:\wintools\ssp\plugin\


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: calendarskinpathlist
Reference0: D:\wintools\ssp\calendar\skin\


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: calendarpluginpathlist
Reference0: D:\wintools\ssp\calendar\plugin\


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: menu.background.bitmap.filename


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: menu.foreground.bitmap.filename


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: menu.background.color.r


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: menu.background.color.g


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: menu.background.color.b


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: menu.foreground.color.r


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: menu.foreground.color.g


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: menu.foreground.color.b


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: username


=====response=====
SHIORI/3.0 200 OK
Value: ユーザさん 
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: installedghostname
Reference0: ．さくら
Reference1: ．さくら 20%
Reference2: Emily/Phase4.5
Reference3: あいすくりん
Reference4: えっじちゃん
Reference5: さとりすとのさんぷるごーすと
Reference6: 奏でる日常の音色
Reference7: 窓の中の世界


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: installedballoonname
Reference0: Balloon for Emily/P4
Reference1: edge
Reference2: SSPデフォルト+
Reference3: SSPデフォルト+
Reference4: ボトル（でふぉ改）


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: installedheadlinename
Reference0: GAME Watch
Reference1: Google ニュース 日本語版
Reference2: ITmedia +D Games
Reference3: ITMedia News
Reference4: SlashDot-JP
Reference5: ねとわくアンテナ
Reference6: ハァハァアンテナ
Reference7: 何かアンテナ
Reference8: 各種スレアンテナ
Reference9: 回収・無償修理等のお知らせ


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: sakura.recommendsites


=====response=====
SHIORI/3.0 200 OK
Value: お茶百科http://ocha.tv/日本紅茶協会http://www.tea-a.gr.jp/knowledge/
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: sakura.portalsites


=====response=====
SHIORI/3.0 200 OK
Value: 作者サイトhttp://nanachi.sakura.ne.jp/index.html作者ブログ（更新履歴など）http://ukananachi.blog98.fc2.com/奏のTwitterBothttps://twitter.com/#!/kanade_ghost
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: kero.recommendsites


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: installedshellname
Reference0: マスターシェル


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: uniqueid
Reference0: ssp_fmo_header_00007b70_002502ca


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
ID: otherghostname
SecurityLevel: local
Charset: UTF-8
Sender: SSP


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: hwnd
Reference0: 24255463346008
Reference1: 1051448408027826


=====response=====
***CallTime:1msec.
SHIORI/3.0 200 OK
Value: \12\n2
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: installedshellname
Reference0: マスターシェル


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnDisplayChange
Reference0: 32
Reference1: 2560
Reference2: 1600


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnBatteryNotify
Reference0: -1
Reference1: -1
Reference2: online
Reference3: no_battery


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnNotifyDressupInfo
Reference0: 1アクセサリCLOSED0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnScheduleTodayNotify


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: installedplugin
Reference0: ghosts(shillest)41f5b44c-c6e1-41af-a86d-fc6061b05b74
Reference1: SwissArmyKnife8F8BCFB8-B27A-456f-9BA0-551484856DDC
Reference2: 共有変数プラグインABED14AF-F34B-4ff2-95B7-30ED37D5802D


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnBoot
Reference0: マスターシェル


=====response=====
***CallTime:2146msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\1\![bind,アクセサリ,CLOSED,0]\![set,otherghosttalk,true]\n\0\s[0]こんばんは。\w7\nユーザさんは晩ご飯食べた？\w9\n\n\s[5]まだだったら、\w4用意するよ。\w6\e
Charset: UTF-8


=====send=====
GET SHIORI/3.0
ID: OnTranslate
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Reference0: \1\s[1512]\1\![bind,アクセサリ,CLOSED,0]\![set,otherghosttalk,true]\n\0\s[0]こんばんは。\w7\nユーザさんは晩ご飯食べた？\w9\n\n\s[5]まだだったら、\w4用意するよ。\w6\e
Reference1: 
Reference2: OnBoot
Reference3: マスターシェル


=====response=====
***CallTime:2msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\1\![bind,アクセサリ,CLOSED,0]\![set,otherghosttalk,true]\n\0\s[0]こんばんは。\w7\nユーザさんは晩ご飯食べた？\w9\n\n\s[5]まだだったら、\w4用意するよ。\w6\e
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking
ID: rateofusegraph
Reference0: 奏でる日常の音色かなでメニューボード911451483.046289boot
Reference1: 窓の中の世界パスタセキュリティホール814668.388167install
Reference2: さとりすとのさんぷるごーすとちぇっかてすた326263.581850install
Reference3: ．さくら 20%．さくらうにゅう214412.523316install
Reference4: ．さくらさくらうにゅう91801.029925install
Reference5: 窓の中の世界パスタセキュリティぼーる11781.018481install
Reference6: えっじちゃんえっじ4330.188820install
Reference7: あいすくりんあいす3150.085827install
Reference8: 窓の中の世界パスタクラックポイント1130.074383install
Reference9: Emily/Phase4.5EmilyTeddy770.040053install
Reference10: 窓の中の世界セキュリティぼーるパスタ240.022887install


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 0
Reference1: 1512
Reference2: 0,0,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 0
Reference1: 1512
Reference2: 1,1512,207,195


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 5
Reference1: 1512
Reference2: 0,5,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 1


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 2


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: homeurl


=====response=====
SHIORI/3.0 200 OK
Value: http://nanachi.sakura.ne.jp/kanade_2nd/
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnterAll
Reference0: 182
Reference1: 123
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnter
Reference0: 182
Reference1: 123
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 182
Reference1: 123
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseHover
Reference0: 180
Reference1: 128
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: tooltip
Reference0: 180
Reference1: 128
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 177
Reference1: 135
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 175
Reference1: 140
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 173
Reference1: 143
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeave
Reference0: 173
Reference1: 144
Reference2: 0
Reference3: 0
Reference4: Shoulder
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnter
Reference0: 173
Reference1: 144
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 172
Reference1: 148
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 170
Reference1: 152
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 169
Reference1: 153
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 164
Reference1: 152
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeave
Reference0: 159
Reference1: 152
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnter
Reference0: 159
Reference1: 152
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 159
Reference1: 152
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseHover
Reference0: 149
Reference1: 149
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: tooltip
Reference0: 149
Reference1: 149
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 144
Reference1: 149
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 134
Reference1: 147
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 133
Reference1: 146
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 135
Reference1: 146
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 137
Reference1: 146
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 139
Reference1: 146
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 140
Reference1: 145
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseDown
Reference0: 140
Reference1: 145
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseUp
Reference0: 140
Reference1: 145
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseClick
Reference0: 140
Reference1: 145
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:2msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnBalloonTimeout
Reference0: \1\s[1512]\1\![bind,アクセサリ,CLOSED,0]\![set,otherghosttalk,true]\n\0\s[0]こんばんは。\w7\nユーザさんは晩ご飯食べた？\w9\n\n\s[5]まだだったら、\w4用意するよ。\w6\e


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 1


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnMouseMove
Reference0: 140
Reference1: 144
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnMouseMove
Reference0: 141
Reference1: 140
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnMouseDown
Reference0: 141
Reference1: 140
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnMouseUp
Reference0: 141
Reference1: 140
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnMouseClick
Reference0: 141
Reference1: 140
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnMouseDoubleClick
BaseID: OnMouseDown
Reference0: 141
Reference1: 140
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\0\s[60]そういうことするんだ。\e
Charset: UTF-8


=====send=====
GET SHIORI/3.0
ID: OnTranslate
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Reference0: \1\s[1512]\0\s[60]そういうことするんだ。\e
Reference1: 
Reference2: OnMouseDoubleClick
Reference3: 14114000Bust0mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\0\s[60]そういうことするんだ。\e
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 142
Reference1: 140
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 60
Reference1: 1512
Reference2: 0,60,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseUp
Reference0: 142
Reference1: 140
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 142
Reference1: 139
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseLeave
Reference0: 146
Reference1: 133
Reference2: 0
Reference3: 0
Reference4: Bust
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseEnter
Reference0: 146
Reference1: 133
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 150
Reference1: 127
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 162
Reference1: 114
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeave
Reference0: 175
Reference1: 101
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeaveAll
Reference0: 175
Reference1: 101
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnterAll
Reference0: 193
Reference1: 91
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnter
Reference0: 193
Reference1: 91
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 193
Reference1: 91
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeave
Reference0: 206
Reference1: 87
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeaveAll
Reference0: 206
Reference1: 87
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnterAll
Reference0: 193
Reference1: 88
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnter
Reference0: 193
Reference1: 88
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 193
Reference1: 88
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseHover
Reference0: 179
Reference1: 83
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: tooltip
Reference0: 179
Reference1: 83
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 175
Reference1: 82
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 174
Reference1: 82
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 171
Reference1: 81
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 164
Reference1: 77
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeave
Reference0: 158
Reference1: 74
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnter
Reference0: 158
Reference1: 74
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 158
Reference1: 74
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 153
Reference1: 70
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 151
Reference1: 67
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 148
Reference1: 64
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeave
Reference0: 145
Reference1: 61
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnter
Reference0: 145
Reference1: 61
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 145
Reference1: 61
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseHover
Reference0: 143
Reference1: 58
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: tooltip
Reference0: 143
Reference1: 58
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 143
Reference1: 57
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 142
Reference1: 57
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 142
Reference1: 55
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 140
Reference1: 52
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 139
Reference1: 51
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseLeave
Reference0: 139
Reference1: 49
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseEnter
Reference0: 139
Reference1: 49
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 139
Reference1: 49
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 139
Reference1: 46
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseMove
Reference0: 139
Reference1: 43
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseDown
Reference0: 139
Reference1: 42
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseUp
Reference0: 139
Reference1: 42
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseClick
Reference0: 139
Reference1: 42
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnMouseDoubleClick
BaseID: OnMouseDown
Reference0: 139
Reference1: 42
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:9msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\0\s[0]メニューだね。\n\n\0\b[2]\*\0\![quicksession,true]基本操作-----------------\n\![*]\q[何か喋って,喋って何か喋って1]\n\![*]\q[話しかける,コミュニケート話しかける2]\n\0\n便利機能------------------\n\![*]\q[天気予報,天気予報情報取得開始天気予報3]\n\![*]\q[ランチャ起動,ランチャ開始手順１ランチャ起動4]\n\![*]\q[タイマー,タイマー機能タイマー5]\n\![*]\q[インターネット検索,インターネット検索インターネット検索6]\n\![*]\q[ニュースを見る,ニュースニュースを見る7]\n\![*]\q[ゴーストエクスプローラ起動,GE起動ゴーストエクスプローラ起動8]\n\![*]\q[音楽再生,音楽再生メニュー音楽再生9]\n\nそのほか------------------\n\![*]\q[各種設定,設定メニュー各種設定10]\n\![*]\q[作者にメッセージを送信する(バグ報告もこちら),拍手送信作者にメッセージを送信する(バグ報告もこちら)11]\n\![*]\q[機能一覧,そのほか機能機能一覧12]\n\n\![*]\q[特に用はないよ,メインメニューを閉じる特に用はないよ13]\_q\n\n１２月２６日（月） ２１：４６\n\1\c\![*]\q[つかいかたのヒント,Tips表示設定つかいかたのヒント14]\n\n土曜日にはお茶会に対応するゴーストさんとお茶会パーティを開けるよ。\n\0\e
Charset: UTF-8


=====send=====
GET SHIORI/3.0
ID: OnTranslate
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Reference0: \1\s[1512]\0\s[0]メニューだね。\n\n\0\b[2]\*\0\![quicksession,true]基本操作-----------------\n\![*]\q[何か喋って,喋って何か喋って1]\n\![*]\q[話しかける,コミュニケート話しかける2]\n\0\n便利機能------------------\n\![*]\q[天気予報,天気予報情報取得開始天気予報3]\n\![*]\q[ランチャ起動,ランチャ開始手順１ランチャ起動4]\n\![*]\q[タイマー,タイマー機能タイマー5]\n\![*]\q[インターネット検索,インターネット検索インターネット検索6]\n\![*]\q[ニュースを見る,ニュースニュースを見る7]\n\![*]\q[ゴーストエクスプローラ起動,GE起動ゴーストエクスプローラ起動8]\n\![*]\q[音楽再生,音楽再生メニュー音楽再生9]\n\nそのほか------------------\n\![*]\q[各種設定,設定メニュー各種設定10]\n\![*]\q[作者にメッセージを送信する(バグ報告もこちら),拍手送信作者にメッセージを送信する(バグ報告もこちら)11]\n\![*]\q[機能一覧,そのほか機能機能一覧12]\n\n\![*]\q[特に用はないよ,メインメニューを閉じる特に用はないよ13]\_q\n\n１２月２６日（月） ２１：４６\n\1\c\![*]\q[つかいかたのヒント,Tips表示設定つかいかたのヒント14]\n\n土曜日にはお茶会に対応するゴーストさんとお茶会パーティを開けるよ。\n\0\e
Reference1: 
Reference2: OnMouseDoubleClick
Reference3: 13942000mouse


=====response=====
***CallTime:5msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\0\s[0]メニューだね。\n\n\0\b[2]\*\0\![quicksession,true]基本操作-----------------\n\![*]\q[何か喋って,喋って何か喋って1]\n\![*]\q[話しかける,コミュニケート話しかける2]\n\0\n便利機能------------------\n\![*]\q[天気予報,天気予報情報取得開始天気予報3]\n\![*]\q[ランチャ起動,ランチャ開始手順１ランチャ起動4]\n\![*]\q[タイマー,タイマー機能タイマー5]\n\![*]\q[インターネット検索,インターネット検索インターネット検索6]\n\![*]\q[ニュースを見る,ニュースニュースを見る7]\n\![*]\q[ゴーストエクスプローラ起動,GE起動ゴーストエクスプローラ起動8]\n\![*]\q[音楽再生,音楽再生メニュー音楽再生9]\n\nそのほか------------------\n\![*]\q[各種設定,設定メニュー各種設定10]\n\![*]\q[作者にメッセージを送信する(バグ報告もこちら),拍手送信作者にメッセージを送信する(バグ報告もこちら)11]\n\![*]\q[機能一覧,そのほか機能機能一覧12]\n\n\![*]\q[特に用はないよ,メインメニューを閉じる特に用はないよ13]\_q\n\n１２月２６日（月） ２１：４６\n\1\c\![*]\q[つかいかたのヒント,Tips表示設定つかいかたのヒント14]\n\n土曜日にはお茶会に対応するゴーストさんとお茶会パーティを開けるよ。\n\0\e
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseUp
Reference0: 139
Reference1: 42
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 0
Reference1: 1512
Reference2: 0,0,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2)
ID: OnMouseMove
Reference0: 138
Reference1: 42
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2)
ID: OnMouseMove
Reference0: 134
Reference1: 44
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2)
ID: OnMouseMove
Reference0: 132
Reference1: 45
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2)
ID: OnMouseMove
Reference0: 130
Reference1: 46
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 129
Reference1: 47
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 128
Reference1: 47
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 126
Reference1: 48
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 124
Reference1: 49
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseLeave
Reference0: 124
Reference1: 50
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseEnter
Reference0: 124
Reference1: 50
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 121
Reference1: 51
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 119
Reference1: 52
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 116
Reference1: 54
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 113
Reference1: 55
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 111
Reference1: 56
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 108
Reference1: 57
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 106
Reference1: 57
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 106
Reference1: 58
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseHover
Reference0: 105
Reference1: 58
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: tooltip
Reference0: 105
Reference1: 58
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 104
Reference1: 58
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 100
Reference1: 58
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 97
Reference1: 58
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseLeave
Reference0: 95
Reference1: 59
Reference2: 0
Reference3: 0
Reference4: Face
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseEnter
Reference0: 95
Reference1: 59
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 95
Reference1: 59
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 91
Reference1: 61
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 88
Reference1: 65
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseLeave
Reference0: 83
Reference1: 70
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseEnter
Reference0: 83
Reference1: 70
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 83
Reference1: 70
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 70
Reference1: 90
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseLeave
Reference0: 64
Reference1: 98
Reference2: 0
Reference3: 0
Reference4: Ear
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseEnter
Reference0: 64
Reference1: 98
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseLeave
Reference0: 61
Reference1: 102
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,choosing,balloon(0=2,1=0)
ID: OnMouseLeaveAll
Reference0: 61
Reference1: 102
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseEnterAll
Reference0: 169
Reference1: 22
Reference2: 0
Reference3: 1
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseEnter
Reference0: 169
Reference1: 22
Reference2: 0
Reference3: 1
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 169
Reference1: 22
Reference2: 0
Reference3: 1
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseLeave
Reference0: 163
Reference1: 26
Reference2: 0
Reference3: 1
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseEnter
Reference0: 163
Reference1: 26
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 149
Reference1: 41
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 137
Reference1: 52
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 131
Reference1: 58
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 126
Reference1: 65
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 121
Reference1: 71
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 118
Reference1: 73
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 118
Reference1: 74
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 116
Reference1: 74
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 115
Reference1: 74
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 115
Reference1: 73
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 114
Reference1: 72
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 112
Reference1: 69
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 109
Reference1: 64
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 106
Reference1: 57
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseMove
Reference0: 100
Reference1: 39
Reference2: 0
Reference3: 1
Reference4: Image
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseLeave
Reference0: 96
Reference1: 20
Reference2: 0
Reference3: 1
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnMouseLeaveAll
Reference0: 96
Reference1: 20
Reference2: 0
Reference3: 1
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter
Reference0: 特に用はないよ
Reference1: メインメニューを閉じる特に用はないよ13


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter
Reference0: 作者にメッセージを送信する(バグ報告もこちら)
Reference1: 拍手送信作者にメッセージを送信する(バグ報告もこちら)11


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter
Reference0: 各種設定
Reference1: 設定メニュー各種設定10


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceHover
Reference0: 話しかける
Reference1: コミュニケート話しかける2


=====response=====
***CallTime:2msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: balloon_tooltip
Reference0: 話しかける
Reference1: コミュニケート話しかける2


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter
Reference0: 何か喋って
Reference1: 喋って何か喋って1


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter
Reference0: 何か喋って
Reference1: 喋って何か喋って1


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter
Reference0: 話しかける
Reference1: コミュニケート話しかける2


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter
Reference0: 何か喋って
Reference1: 喋って何か喋って1


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceEnter


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: choosing,balloon(0=2,1=0)
ID: OnChoiceSelectEx
Reference0: 何か喋って
Reference1: 喋って何か喋って1


=====response=====
***CallTime:15msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\1\n[half]\0\s[25]ジャムをたべながら\_a[紅茶]紅茶\_aをいただくのがロシアンティーなんだけど、\w9\n\s[5]ジャムを溶かしていただく\_a[紅茶]紅茶\_aもなんの不思議もなく美味しいんだよね。\e
Charset: UTF-8


=====send=====
GET SHIORI/3.0
ID: OnTranslate
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Reference0: \1\s[1512]\1\n[half]\0\s[25]ジャムをたべながら\_a[紅茶]紅茶\_aをいただくのがロシアンティーなんだけど、\w9\n\s[5]ジャムを溶かしていただく\_a[紅茶]紅茶\_aもなんの不思議もなく美味しいんだよね。\e
Reference1: 
Reference2: OnChoiceSelectEx
Reference3: 何か喋って喋って何か喋って1


=====response=====
***CallTime:1msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\1\n[half]\0\s[25]ジャムをたべながら\_a[紅茶]紅茶\_aをいただくのがロシアンティーなんだけど、\w9\n\s[5]ジャムを溶かしていただく\_a[紅茶]紅茶\_aもなんの不思議もなく美味しいんだよね。\e
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 25
Reference1: 1512
Reference2: 0,25,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 5
Reference1: 1512
Reference2: 0,5,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnChoiceEnter
Reference0: 紅茶
Reference1: 紅茶


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnChoiceHover
Reference0: 紅茶
Reference1: 紅茶


=====response=====
***CallTime:2msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: balloon_tooltip
Reference0: 紅茶
Reference1: 紅茶


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 1


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnChoiceEnter


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnAnchorSelectEx
Reference0: 紅茶
Reference1: 紅茶


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnAnchorSelect
Reference0: 紅茶


=====response=====
SHIORI/3.0 200 OK
Value: \1\s[1512]\0\s[0]ん、\w3\_a[紅茶]紅茶\_a飲みたい？\w5\n\s[1]ケーキ買ってきてくれたら良いよ。\w8\n\n…\w3\s[25]なんて冗談だよ。\w8\s[5] はい、\_a[紅茶]紅茶\_a。\e
Charset: UTF-8


=====send=====
GET SHIORI/3.0
ID: OnTranslate
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Reference0: \1\s[1512]\0\s[0]ん、\w3\_a[紅茶]紅茶\_a飲みたい？\w5\n\s[1]ケーキ買ってきてくれたら良いよ。\w8\n\n…\w3\s[25]なんて冗談だよ。\w8\s[5] はい、\_a[紅茶]紅茶\_a。\e
Reference1: 
Reference2: OnAnchorSelect
Reference3: 紅茶


=====response=====
***CallTime:1msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\0\s[0]ん、\w3\_a[紅茶]紅茶\_a飲みたい？\w5\n\s[1]ケーキ買ってきてくれたら良いよ。\w8\n\n…\w3\s[25]なんて冗談だよ。\w8\s[5] はい、\_a[紅茶]紅茶\_a。\e
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 0
Reference1: 1512
Reference2: 0,0,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 1
Reference1: 1512
Reference2: 0,1,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseEnterAll
Reference0: 41
Reference1: 186
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseEnter
Reference0: 41
Reference1: 186
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 41
Reference1: 186
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseLeave
Reference0: 44
Reference1: 187
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseEnter
Reference0: 44
Reference1: 187
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 45
Reference1: 188
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 45
Reference1: 192
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 46
Reference1: 198
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 46
Reference1: 205
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 46
Reference1: 211
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 48
Reference1: 216
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 50
Reference1: 221
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 53
Reference1: 226
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 56
Reference1: 231
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 58
Reference1: 237
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 61
Reference1: 241
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseLeave
Reference0: 62
Reference1: 244
Reference2: 0
Reference3: 0
Reference4: Rribbon
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseEnter
Reference0: 62
Reference1: 244
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 62
Reference1: 244
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 64
Reference1: 245
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 63
Reference1: 245
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 25
Reference1: 1512
Reference2: 0,25,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseDown
Reference0: 63
Reference1: 245
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 1
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseUp
Reference0: 63
Reference1: 245
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 1
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseClick
Reference0: 63
Reference1: 245
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 1
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sakura.popupmenu.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sakura.popupmenu.type


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: recommendrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sakura.recommendbuttoncaption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: recommendrootbutton.caption


=====response=====
SHIORI/3.0 200 OK
Value: お茶の情報サイト(&R)
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: portalrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sakura.portalbuttoncaption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: portalrootbutton.caption


=====response=====
SHIORI/3.0 200 OK
Value: このゴースト関連のリンク(&P)
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: updatebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: updatebuttoncaption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sakura.updatebuttoncaption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: updatebutton.caption


=====response=====
SHIORI/3.0 200 OK
Value: オンラインアップデート(&U)
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: readmebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: readmebuttoncaption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: readmebutton.caption


=====response=====
SHIORI/3.0 200 OK
Value: readme.txtの表示(&R)
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: vanishbuttonvisible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: vanishbutton.visible


=====response=====
SHIORI/3.0 200 OK
Value: 1 
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: vanishbuttoncaption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: vanishbutton.caption


=====response=====
SHIORI/3.0 200 OK
Value: アンインストール(&F)
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: aistatebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: getaistate


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: aistatebutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sakura.recommendsites


=====response=====
SHIORI/3.0 200 OK
Value: お茶百科http://ocha.tv/日本紅茶協会http://www.tea-a.gr.jp/knowledge/
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sakura.portalsites


=====response=====
SHIORI/3.0 200 OK
Value: 作者サイトhttp://nanachi.sakura.ne.jp/index.html作者ブログ（更新履歴など）http://ukananachi.blog98.fc2.com/奏のTwitterBothttps://twitter.com/#!/kanade_ghost
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: headlinesenserootbutton.visible


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: headlinesenserootbutton.caption


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: pluginrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: pluginrootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: biffbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: biffbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: shellscalerootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: shellscalerootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: utilityrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: utilityrootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: calendarbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: calendarbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: messengerbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: messengerbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sntpbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: sntpbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: ghostexplorerbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: ghostexplorerbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: scriptlogbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: scriptlogbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: addressbarbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: addressbarbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: openfilebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: openfilebutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: openfolderbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: openfolderbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: updateplatformbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: updateplatformbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: purgeghostcachebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: purgeghostcachebutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: updatefmobutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: updatefmobutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: reloadinfobutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: reloadinfobutton.caption


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchreloadbutton.visible


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchreloadbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: leavepassivebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: leavepassivebutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchreloadtempghostbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchreloadtempghostbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchmovetodefaultpositionbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchmovetodefaultpositionbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: resetballoonpositionbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: resetballoonpositionbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: closeballoonbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: closeballoonbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: duibutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: duibutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: configurationrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: configurationrootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: configurationbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: configurationbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: configurationghostbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: configurationghostbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: charsetbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: charsetbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchproxybutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchproxybutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchtalkghostbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchtalkghostbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchcompatiblemodebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: switchcompatiblemodebutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: ghostrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: ghostrootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: callghostrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: callghostrootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: shellrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: shellrootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: dressuprootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: dressuprootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: balloonrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: balloonrootbutton.caption


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: historyrootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: historyrootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: ghosthistorybutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: ghosthistorybutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: callghosthistorybutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: callghosthistorybutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: balloonhistorybutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: balloonhistorybutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: headlinesensehistorybutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: headlinesensehistorybutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: pluginhistorybutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: pluginhistorybutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: inforootbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: inforootbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: firststaffbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: firststaffbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: readmebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: readmebutton.caption


=====response=====
SHIORI/3.0 200 OK
Value: readme.txtの表示(&R)
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: helpbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: helpbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: rateofusebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: rateofusebutton.caption


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: systeminfobutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: systeminfobutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: hidebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: hidebutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: closebutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: closebutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: quitbutton.visible


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: quitbutton.caption


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseLeave
Reference0: 62
Reference1: 245
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseLeaveAll
Reference0: 62
Reference1: 245
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 5
Reference1: 1512
Reference2: 0,5,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 1
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnCloseAll
BaseID: OnClose


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: OnClose
Reference0: user


=====response=====
***CallTime:1msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\0\s[6]お茶の一杯くらい、\w5\n飲んでいっても良かったんじゃない？\-
Charset: UTF-8


=====send=====
GET SHIORI/3.0
ID: OnTranslate
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Reference0: \1\s[1512]\0\s[6]お茶の一杯くらい、\w5\n飲んでいっても良かったんじゃない？\-
Reference1: 
Reference2: OnClose
Reference3: user


=====response=====
***CallTime:1msec.
SHIORI/3.0 200 OK
Value: \1\s[1512]\0\s[6]お茶の一杯くらい、\w5\n飲んでいっても良かったんじゃない？\-\e
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseEnterAll
Reference0: 76
Reference1: 239
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseEnter
Reference0: 76
Reference1: 239
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 76
Reference1: 239
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSurfaceChange
Reference0: 6
Reference1: 1512
Reference2: 0,6,260,400


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 77
Reference1: 239
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 79
Reference1: 239
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 81
Reference1: 239
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 83
Reference1: 238
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 89
Reference1: 236
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 100
Reference1: 231
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 116
Reference1: 222
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMinuteChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
***CallTime:1msec.
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 135
Reference1: 212
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 158
Reference1: 199
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 178
Reference1: 179
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseLeave
Reference0: 189
Reference1: 166
Reference2: 0
Reference3: 0
Reference4: 
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseEnter
Reference0: 189
Reference1: 166
Reference2: 0
Reference3: 0
Reference4: Hair
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseMove
Reference0: 202
Reference1: 149
Reference2: 0
Reference3: 0
Reference4: Hair
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseLeave
Reference0: 206
Reference1: 142
Reference2: 0
Reference3: 0
Reference4: Hair
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
GET SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnMouseLeaveAll
Reference0: 206
Reference1: 142
Reference2: 0
Reference3: 0
Reference4: Hair
Reference5: 0
Reference6: mouse


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: talking,balloon(0=0)
ID: OnSecondChange
Reference0: 1
Reference1: 0
Reference2: 0
Reference3: 0
Reference4: 0


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
Status: balloon(0=0)
ID: rateofusegraph
Reference0: 奏でる日常の音色かなでメニューボード911451483.046289install
Reference1: 窓の中の世界パスタセキュリティホール814668.388167install
Reference2: さとりすとのさんぷるごーすとちぇっかてすた326263.581850install
Reference3: ．さくら 20%．さくらうにゅう214412.523316install
Reference4: ．さくらさくらうにゅう91801.029925install
Reference5: 窓の中の世界パスタセキュリティぼーる11781.018481install
Reference6: えっじちゃんえっじ4330.188820install
Reference7: あいすくりんあいす3150.085827install
Reference8: 窓の中の世界パスタクラックポイント1130.074383install
Reference9: Emily/Phase4.5EmilyTeddy770.040053install
Reference10: 窓の中の世界セキュリティぼーるパスタ240.022887install


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
NOTIFY SHIORI/3.0
Charset: UTF-8
Sender: SSP
SecurityLevel: local
ID: OnDestroy
Reference0: 


=====response=====
SHIORI/3.0 204 No Content
Charset: UTF-8


=====send=====
(unload call)
=====send=====
(free library)
