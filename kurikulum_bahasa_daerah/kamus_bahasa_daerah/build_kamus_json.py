import json

dataset = {
    "versi": "6.0.0",
    "judul": "Kamus Digital Leksikografis Akbar Bahasa Daerah Nusantara (Edisi 80+ Entri Lengkap)",
    "total_entri": 0,
    "bahasa_didukung": [
        "jawa", "sunda", "bali", "minang", "bugis", "makassar",
        "batak_toba", "aceh", "banjar", "lampung", "papua"
    ],
    "entri": []
}

entries = [
    # 1. NUMERALIA (BILANGAN)
    {
        "indonesia": "satu", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "siji", "krama_inggil": "setunggal", "aksara": "ꦱꦼꦠꦸꦁꦒꦭ", "contoh": "Setunggal tambah setunggal dados kalih."},
            "sunda": {"loma": "hiji", "lemes_batur": "hiji", "aksara": "ᮠᮤᮏᮤ", "contoh": "Hiji dinten sim kuring jalan-jalan."},
            "bali": {"andap": "sa / besik", "alus_singgih": "siki", "aksara": "ᬲᬶᬓᬶ", "contoh": "Matur suksma siki puniki."},
            "minang": {"kata": "ciek / satu", "contoh": "Galeh balanjo ciek ciek sajo."},
            "bugis": {"kata": "seddi", "aksara": "ᨔᨛᨉᨗ", "contoh": "Seddi atinna mabbarakka."},
            "makassar": {"kata": "se're", "contoh": "Se'reji tau baji'."},
            "batak_toba": {"kata": "sada", "aksara": "ᯘᯑ", "contoh": "Sada do roha ni hita."},
            "aceh": {"kata": "sa", "contoh": "Sa boh saban raseuki."},
            "banjar": {"kata": "asa / satu", "contoh": "Asa liwar rami banar."},
            "lampung": {"kata": "sai", "aksara": "ᰞᰦᰤ", "contoh": "Lampung sai bumi ruwa jurai."},
            "papua": {"kata": "satu", "contoh": "Satu hati satu rasa kitorang samua."}
        }
    },
    {
        "indonesia": "dua", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "loro", "krama_inggil": "kalih", "aksara": "ꦏꦭꦶꦃ", "contoh": "Gadhah putra kalih jaler lan estri."},
            "sunda": {"loma": "dua", "lemes_batur": "dua", "aksara": "ᮓᮥᮃ", "contoh": "Duaan sareng pun bojo."},
            "bali": {"andap": "dua", "alus_singgih": "kalih", "aksara": "ᬓᬮᬶᬄ", "contoh": "Magenah ring kalih genah."},
            "minang": {"kata": "duo", "contoh": "Duo tigo kuciang balari."},
            "bugis": {"kata": "dua", "aksara": "ᨉᨘᨕ", "contoh": "Dua bola masiga'."},
            "makassar": {"kata": "rua", "contoh": "Rua tau a'jama."},
            "batak_toba": {"kata": "dua", "aksara": "ᯑᯮᯀ", "contoh": "Dua halak dongan tubu."},
            "aceh": {"kata": "dua", "contoh": "Dua droe ureueng gampong."},
            "banjar": {"kata": "dua", "contoh": "Dua jukung lalu di sungai."},
            "lampung": {"kata": "ghua / ruwa", "aksara": "ᰛᰪᰓᰦ", "contoh": "Ruwa jurai suku Lampung."},
            "papua": {"kata": "dua", "contoh": "Dua tifa berbunyi serempak."}
        }
    },
    {
        "indonesia": "tiga", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "telu", "krama_inggil": "tiga", "aksara": "ꦠꦶꦒ", "contoh": "Tiga wulan laminipun sinau."},
            "sunda": {"loma": "tilu", "lemes_batur": "tilu", "aksara": "ᮒᮤᮜᮥ", "contoh": "Tilu puluh dinten puasa."},
            "bali": {"andap": "telu", "alus_singgih": "tiga", "aksara": "ᬢᬶᬕ", "contoh": "Tri Hita Karana tiga kahanan."},
            "minang": {"kata": "tigo", "contoh": "Luhak Nan Tigo adaik Minang."},
            "bugis": {"kata": "tellu", "aksara": "ᨈᨛᨒᨘ", "contoh": "Tellu cappa'na Ogi'e."},
            "makassar": {"kata": "tallu", "contoh": "Tallu bintoeng marak."},
            "batak_toba": {"kata": "tolu", "aksara": "ᯖᯬᯞᯮ", "contoh": "Dalihan Na Tolu pilar adat."},
            "aceh": {"kata": "lhee", "contoh": "Lhee sagoe tanoh rencong."},
            "banjar": {"kata": "talu / tiga", "contoh": "Talu ikung iwak patin."},
            "lampung": {"kata": "tigo / telu", "aksara": "ᰊᰦᰜᰪ", "contoh": "Tiga pekon mehempur."},
            "papua": {"kata": "tiga", "contoh": "Tiga perahu dayung melaju."}
        }
    },
    {
        "indonesia": "empat", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "papat", "krama_inggil": "sekawan", "aksara": "ꦱꦼꦏꦮꦤ", "contoh": "Kiblat papat lima pancer."},
            "sunda": {"loma": "opat", "lemes_batur": "opat", "aksara": "ᮇᮕᮒ", "contoh": "Opat juru mata angin."},
            "bali": {"andap": "papat", "alus_singgih": "patpat", "aksara": "ᬧᬢᬧᬢ", "contoh": "Katurang ring patpat genah."},
            "minang": {"kata": "ampek", "contoh": "Kato Nan Ampek pegangan tutur."},
            "bugis": {"kata": "eppa'", "aksara": "ᨕᨛᨄ", "contoh": "Sulapa eppa' wanuwa."},
            "makassar": {"kata": "appa'", "contoh": "Appa' sulapa'na butta."},
            "batak_toba": {"kata": "opat", "aksara": "ᯀᯬᯇᯖ", "contoh": "Opat desa naualu."},
            "aceh": {"kata": "peuet", "contoh": "Tuha peut pimpinan gampong."},
            "banjar": {"kata": "ampat", "contoh": "Ampat panjuru mata angin."},
            "lampung": {"kata": "pak", "aksara": "ᰎᰦᰀ", "contoh": "Pak tiyuh bersatu."},
            "papua": {"kata": "empat", "contoh": "Empat pulau Raja Ampat."}
        }
    },
    {
        "indonesia": "lima", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "lima", "krama_inggil": "gangsal", "aksara": "ꦒꦁꦱꦭ", "contoh": "Gangsal perkawis wajib katindakaken."},
            "sunda": {"loma": "lima", "lemes_batur": "lima", "aksara": "ᮜᮤᮙ", "contoh": "Pancasila lima dasar nagara."},
            "bali": {"andap": "lima", "alus_singgih": "panca", "aksara": "ᬧᬜᬘ", "contoh": "Panca Yadnya panca kurban suci."},
            "minang": {"kata": "limo", "contoh": "Rukun Islam nan limo pacangguang."},
            "bugis": {"kata": "lima", "aksara": "ᨒᨗᨆ", "contoh": "Lima rupa pangadereng."},
            "makassar": {"kata": "lima", "contoh": "Lima pilar kasalamakkang."},
            "batak_toba": {"kata": "lima", "aksara": "ᯞᯪᯔ", "contoh": "Lima tali si pir ni tondi."},
            "aceh": {"kata": "limong", "contoh": "Limong watee sembahyang."},
            "banjar": {"kata": "lima", "contoh": "Lima waktu jangan ditinggalakan."},
            "lampung": {"kata": "lima", "aksara": "ᰜᰤᰕᰦ", "contoh": "Lima prinsip piil pesenggiri."},
            "papua": {"kata": "lima", "contoh": "Lima jari tangan bersatu."}
        }
    },
    {
        "indonesia": "enam", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "enem", "krama_inggil": "enem", "aksara": "ꦲꦼꦤꦼꦩ", "contoh": "Enem wulan sampun lumampah."},
            "sunda": {"loma": "genep", "lemes_batur": "genep", "aksara": "ᮌᮨᮔᮨᮕ", "contoh": "Genep poe lilana damel."},
            "bali": {"andap": "enem", "alus_singgih": "nem", "aksara": "ᬦᬾᬫ", "contoh": "Sad Ripu nem musuh ring angga."},
            "minang": {"kata": "anam", "contoh": "Anam tanggo rumah adaik."},
            "bugis": {"kata": "enneng", "aksara": "ᨕᨛᨊᨛ", "contoh": "Enneng esso mabbalu'."},
            "makassar": {"kata": "annang", "contoh": "Annang wattu asambayang."},
            "batak_toba": {"kata": "onom", "aksara": "ᯀᯬᯉᯬᯔ", "contoh": "Onom ari ma lelengna."},
            "aceh": {"kata": "nam", "contoh": "Nam uroe geumeukureung."},
            "banjar": {"kata": "anam", "contoh": "Anam hari ba-tulakan."},
            "lampung": {"kata": "enem", "aksara": "ᰀᰠᰀ", "contoh": "Enem lamban mehempur."},
            "papua": {"kata": "enam", "contoh": "Enam orang naik perahu."}
        }
    },
    {
        "indonesia": "tujuh", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "pitu", "krama_inggil": "pitu", "aksara": "ꦥꦶꦠꦸ", "contoh": "Pitulas Agustus dinten kamardikan."},
            "sunda": {"loma": "tujuh", "lemes_batur": "tujuh", "aksara": "ᮒᮥᮏᮥᮂ", "contoh": "Tujuh likur wengi qadar."},
            "bali": {"andap": "pitu", "alus_singgih": "sapta", "aksara": "ᬧᬶᬢᬸ", "contoh": "Sapta Loka pitu genah swarga."},
            "minang": {"kata": "tujuah", "contoh": "Tujuah koto panuah barisi."},
            "bugis": {"kata": "pitu", "aksara": "ᨄᨗᨈᨘ", "contoh": "Pitu arung matoa."},
            "makassar": {"kata": "tuju", "contoh": "Tuju bintoeng makelle'."},
            "batak_toba": {"kata": "pitu", "aksara": "ᯇᯪᯖᯮ", "contoh": "Pitu sundut martua."},
            "aceh": {"kata": "tujoh", "contoh": "Tujoh uroe kenduri adat."},
            "banjar": {"kata": "pitu / tujuh", "contoh": "Pitu malam batamat quran."},
            "lampung": {"kata": "pitu", "aksara": "ᰎᰤᰊᰪ", "contoh": "Pitu jurai bujang gadis."},
            "papua": {"kata": "tujuh", "contoh": "Tujuh suku di Lembah Baliem."}
        }
    },
    {
        "indonesia": "delapan", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "wolu", "krama_inggil": "wolu", "aksara": "ꦮꦺꦴꦭꦸ", "contoh": "Asta Brata wolu ajaran luhur."},
            "sunda": {"loma": "dalapan", "lemes_batur": "dalapan", "aksara": "ᮓᮜᮕᮔ", "contoh": "Dalapan belas taun umurna."},
            "bali": {"andap": "kutus", "alus_singgih": "asta", "aksara": "ᬓᬸᬢᬸᬲ", "contoh": "Asta Dala kutus layon padma."},
            "minang": {"kata": "salapan / dalapan", "contoh": "Dalapan suku nan badunsanak."},
            "bugis": {"kata": "arua", "aksara": "ᨕᨑᨘᨕ", "contoh": "Arua tau madduta."},
            "makassar": {"kata": "karua", "contoh": "Karua juku' nialle."},
            "batak_toba": {"kata": "ualu", "aksara": "ᯥᯀᯞᯮ", "contoh": "Desa naualu desa panjuru."},
            "aceh": {"kata": "lapan", "contoh": "Lapan kuta pertahanan."},
            "banjar": {"kata": "walu", "contoh": "Walu buting buah mangga."},
            "lampung": {"kata": "walu", "aksara": "ᰓᰦᰜᰪ", "contoh": "Walu marga pepadun."},
            "papua": {"kata": "delapan", "contoh": "Delapan pemuda memegang tombak."}
        }
    },
    {
        "indonesia": "sembilan", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "sanga", "krama_inggil": "sanga", "aksara": "ꦱꦔ", "contoh": "Wali Sanga nyebaraken kabecikan."},
            "sunda": {"loma": "salapan", "lemes_batur": "salapan", "aksara": "ᮞᮜᮕᮔ", "contoh": "Salapan bulan ngakandung."},
            "bali": {"andap": "sia", "alus_singgih": "nawa", "aksara": "ᬲᬶᬬ", "contoh": "Nawa Sanga sanga dewata penjaga."},
            "minang": {"kata": "sambilan", "contoh": "Sambilan ruang Istano Basa Pagaruyuang."},
            "bugis": {"kata": "asera", "aksara": "ᨕᨔᨛᨑ", "contoh": "Asera tau matoa."},
            "makassar": {"kata": "salapang", "contoh": "Salapang gallarrang Bantaeng."},
            "batak_toba": {"kata": "sia", "aksara": "ᯘᯪᯀ", "contoh": "Sia boru manjalo ulos."},
            "aceh": {"kata": "sikureueng", "contoh": "Sikureueng sagoe Aceh meutaloe."},
            "banjar": {"kata": "sanga / sambilan", "contoh": "Sanga ikung anak itik."},
            "lampung": {"kata": "siwa", "aksara": "ᰞᰤᰓᰦ", "contoh": "Siwa kebandaran Saibatin."},
            "papua": {"kata": "sembilan", "contoh": "Sembilan klan bermusyawarah."}
        }
    },
    {
        "indonesia": "sepuluh", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "sepuluh", "krama_inggil": "sedasa", "aksara": "ꦱꦼꦢꦱ", "contoh": "Sedasa dinten laminipun pesta."},
            "sunda": {"loma": "sapuluh", "lemes_batur": "sapuluh", "aksara": "ᮞᮕᮥᮜᮥᮂ", "contoh": "Sapuluh taun masantren."},
            "bali": {"andap": "dasa", "alus_singgih": "sadasa", "aksara": "ᬤᬲ", "contoh": "Dasa Mala dasa solah nenten becik."},
            "minang": {"kata": "sapuluah", "contoh": "Sapuluah jari kito susun."},
            "bugis": {"kata": "seppulo", "aksara": "ᨔᨛᨄᨘᨒᨚ", "contoh": "Seppulo taung mabbalanca."},
            "makassar": {"kata": "sampulo", "contoh": "Sampulo ana' makkuraga."},
            "batak_toba": {"kata": "sampulu", "aksara": "ᯘᯔ᯲ᯇᯮᯞᯮ", "contoh": "Sampulu jari ni tanganta."},
            "aceh": {"kata": "siploh", "contoh": "Siploh boh batee asan."},
            "banjar": {"kata": "sapuluh", "contoh": "Sapuluh ribuan duitnya."},
            "lampung": {"kata": "puluh / sapuluh", "aksara": "ᰎᰪᰜᰪᰝ", "contoh": "Sapuluh bilik lamban."},
            "papua": {"kata": "sepuluh", "contoh": "Sepuluh noken berisi ubi."}
        }
    },
    {
        "indonesia": "seratus", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "satus", "krama_inggil": "satunggalatus", "aksara": "ꦱꦠꦸꦱ", "contoh": "Satunggalatus taun yuswanipun."},
            "sunda": {"loma": "saratus", "lemes_batur": "saratus", "aksara": "ᮞᮛᮒᮥᮞ", "contoh": "Saratus kilo tebihna."},
            "bali": {"andap": "satus", "alus_singgih": "satunggil satus", "aksara": "ᬲᬢᬸᬲ", "contoh": "Satus sasih magenah ring gria."},
            "minang": {"kata": "saratuih", "contoh": "Saratuih urang baralek gadang."},
            "bugis": {"kata": "serratu'", "aksara": "ᨔᨛᨑᨈᨘ", "contoh": "Serratu' tedong riolo."},
            "makassar": {"kata": "saratu'", "contoh": "Saratu' juku' pinasa."},
            "batak_toba": {"kata": "saratus", "aksara": "ᯘᯒᯖᯮᯘ", "contoh": "Saratus hali martangiang."},
            "aceh": {"kata": "sireutoh", "contoh": "Sireutoh reusam meugisa."},
            "banjar": {"kata": "saratus", "contoh": "Saratus gantang banih."},
            "lampung": {"kata": "seratus / satos", "aksara": "ᰞᰦᰊᰩᰠ", "contoh": "Seratus batang lada."},
            "papua": {"kata": "seratus", "contoh": "Seratus warga kumpul di lapangan."}
        }
    },
    {
        "indonesia": "seribu", "kelas_kata": "numeralia", "kategori": "numeralia_bilangan",
        "daerah": {
            "jawa": {"ngoko": "seewu", "krama_inggil": "setunggalewu", "aksara": "ꦱꦲꦺꦮꦸ", "contoh": "Lawang Sewu monumen bersejarah."},
            "sunda": {"loma": "sarebu", "lemes_batur": "sarebu", "aksara": "ᮞᮛᮨᮘᮥ", "contoh": "Pulo Sarebu endah pisan."},
            "bali": {"andap": "siu", "alus_singgih": "sasasih siu", "aksara": "ᬲᬶᬬᬸ", "contoh": "Siu canang katurang suci."},
            "minang": {"kata": "saribu", "contoh": "Kawasan Saribu Rumah Gadang Solok Selatan."},
            "bugis": {"kata": "sasebbu", "aksara": "ᨔᨔᨛᨅᨘ", "contoh": "Sasebbu lopi sompe'."},
            "makassar": {"kata": "sasa'bu", "contoh": "Sasa'bu pallontara'."},
            "batak_toba": {"kata": "saribu", "aksara": "ᯘᯒᯪᯅᯮ", "contoh": "Dolok Saribu tano hajadion."},
            "aceh": {"kata": "siribee", "contoh": "Siribee krak haba jameun."},
            "banjar": {"kata": "saribu", "contoh": "Kandangan Kota Sasirangan Saribu Sungai."},
            "lampung": {"kata": "seribu", "aksara": "ᰞᰦᰛᰤᰓᰪ", "contoh": "Seribu bukit Way Kanan."},
            "papua": {"kata": "seribu", "contoh": "Kepulauan seribu pesona Papua."}
        }
    },

    # 2. ADJEKTIVA WARNA
    {
        "indonesia": "putih", "kelas_kata": "adjektiva", "kategori": "adjektiva_warna",
        "daerah": {
            "jawa": {"ngoko": "putih", "krama_inggil": "pethak", "aksara": "ꦥꦼꦛꦏ", "contoh": "Kain mori pethak kangge nyerat bathik."},
            "sunda": {"loma": "bodas", "lemes_batur": "bodas", "aksara": "ᮘᮧᮓᮞ", "contoh": "Kawah Bodas Ciwidey asri tur endah."},
            "bali": {"andap": "putih", "alus_singgih": "seta / petak", "aksara": "ᬧᬾᬢᬓ", "contoh": "Wastra petak busana sembahyang suci."},
            "minang": {"kata": "putiah", "contoh": "Aia janiah sayak putiah."},
            "bugis": {"kata": "kute'", "aksara": "ᨀᨘᨈᨙ", "contoh": "Baju kute' ri pesta bunting."},
            "makassar": {"kata": "kebo'", "contoh": "Tamparang Kassi' Kebo' Bira."},
            "batak_toba": {"kata": "bontar", "aksara": "ᯅᯬᯉ᯲ᯖᯒ", "contoh": "Ulos ragidup na bontar."},
            "aceh": {"kata": "puteh", "contoh": "Kupiah puteh lambang kasucian."},
            "banjar": {"kata": "putih", "contoh": "Kain putih ba-sulap langkar."},
            "lampung": {"kata": "handak", "aksara": "ᰝᰦᰍᰌᰦᰀ", "contoh": "Tapis handak benang emas."},
            "papua": {"kata": "putih", "contoh": "Pasir putih pantai Biak bersih sekali."}
        }
    },
    {
        "indonesia": "hitam", "kelas_kata": "adjektiva", "kategori": "adjektiva_warna",
        "daerah": {
            "jawa": {"ngoko": "ireng", "krama_inggil": "cemeng", "aksara": "ꦕꦼꦩꦼꦁ", "contoh": "Kopi cemeng tanpa gula raosipun mantep."},
            "sunda": {"loma": "hideung", "lemes_batur": "hideung", "aksara": "ᮠᮤᮓᮩᮀ", "contoh": "Ketan hideung pulen didamel bubur."},
            "bali": {"andap": "selem", "alus_singgih": "kresna", "aksara": "ᬲᬾᬮᬾᬫ", "contoh": "Wastra poleng selem putih keseimbangan alam."},
            "minang": {"kata": "itam", "contoh": "Kopi itam Bukittinggi harum aroma."},
            "bugis": {"kata": "bolong", "aksara": "ᨅᨚᨒᨚ", "contoh": "Tedong bonga bolong mapaccing."},
            "makassar": {"kata": "le'leng", "contoh": "Baju bodo le'leng anggun."},
            "batak_toba": {"kata": "birong", "aksara": "ᯅᯪᯒᯬᯰ", "contoh": "Batu birong di topi ni tao."},
            "aceh": {"kata": "itam", "contoh": "Ija itam selempang ulama."},
            "banjar": {"kata": "hirang", "contoh": "Kopi hirang panas kawa dicucul wadai."},
            "lampung": {"kata": "halom", "aksara": "ᰝᰦᰜᰩᰕ", "contoh": "Lada halom komoditas unggul Lampung."},
            "papua": {"kata": "hitam", "contoh": "Batu hitam kali untuk upacara bakar batu."}
        }
    },
    {
        "indonesia": "merah", "kelas_kata": "adjektiva", "kategori": "adjektiva_warna",
        "daerah": {
            "jawa": {"ngoko": "abang", "krama_inggil": "abrit", "aksara": "ꦲꦧꦿꦶꦠ", "contoh": "Gula abang asil nira klapa kemurnian."},
            "sunda": {"loma": "beureum", "lemes_batur": "beureum", "aksara": "ᮘᮩᮛᮩᮙ", "contoh": "Gula beureum Cianjur amis pisan."},
            "bali": {"andap": "barak", "alus_singgih": "rakta", "aksara": "ᬩᬭꦏ", "contoh": "Bunga pucuk barak katurang ring padmasana."},
            "minang": {"kata": "sirah", "contoh": "Songket sirah ameh Minangkabau."},
            "bugis": {"kata": "cella'", "aksara": "ᨌᨛᨒ", "contoh": "Lipa' sabbe cella' wanuwa."},
            "makassar": {"kata": "eja", "contoh": "Juku' eja bakar Paotere."},
            "batak_toba": {"kata": "rara", "aksara": "ᯒᯒ", "contoh": "Ulos sirara warna ni barani."},
            "aceh": {"kata": "mirah", "contoh": "Kupiah meukutob meucorak mirah."},
            "banjar": {"kata": "habang", "contoh": "Wadai habang sarimuka manis."},
            "lampung": {"kata": "suluh", "aksara": "ᰞᰪᰜᰪᰝ", "contoh": "Kain tapis corak suluh."},
            "papua": {"kata": "merah", "contoh": "Burung Cenderawasih merah menari indah."}
        }
    },
    {
        "indonesia": "kuning", "kelas_kata": "adjektiva", "kategori": "adjektiva_warna",
        "daerah": {
            "jawa": {"ngoko": "kuning", "krama_inggil": "jenar", "aksara": "ꦗꦼꦤꦂ", "contoh": "Sega gurih jenar kagem tumpengan slametan."},
            "sunda": {"loma": "koneng", "lemes_batur": "koneng", "aksara": "ᮊᮧᮔᮨᮀ", "contoh": "Sangu koneng pulen sareng hayam bekakak."},
            "bali": {"andap": "kuning", "alus_singgih": "jenar", "aksara": "ᬓᬸᬦᬶᬗ", "contoh": "Bunga cempaka kuning arum gandanyane."},
            "minang": {"kata": "kuniang", "contoh": "Baju kilek kuniang ameh suntiang."},
            "bugis": {"kata": "maridi", "aksara": "ᨆᨑᨗᨉᨗ", "contoh": "Warna maridi pajang lolang."},
            "makassar": {"kata": "kiring", "contoh": "Baju bodo kiring malepang."},
            "batak_toba": {"kata": "gorsing", "aksara": "ᯎᯬᯒ᯲ᯘᯪᯰ", "contoh": "Bunga na gorsing di ladang na balga."},
            "aceh": {"kata": "kuneng", "contoh": "Bu kuneng keu peusampe hajatan."},
            "banjar": {"kata": "kuning", "contoh": "Nasi kuning banjar pakai iwak haruan."},
            "lampung": {"kata": "kuneng", "aksara": "ᰀᰪᰍᰤᰅ", "contoh": "Kain tapis kuneng benang mas."},
            "papua": {"kata": "kuning", "contoh": "Bulu kuning burung kasuari menawan."}
        }
    },
    {
        "indonesia": "hijau", "kelas_kata": "adjektiva", "kategori": "adjektiva_warna",
        "daerah": {
            "jawa": {"ngoko": "ijo", "krama_inggil": "ijem", "aksara": "ꦲꦶꦗꦼꦩ", "contoh": "Sabin ijem royo-royo subur makmur."},
            "sunda": {"loma": "hejo", "lemes_batur": "hejo", "aksara": "ᮠᮦᮏᮧ", "contoh": "Kebon teh hejo ngampar di Pagelaran."},
            "bali": {"andap": "gadang", "alus_singgih": "gadang", "aksara": "ᬕᬤᬗ", "contoh": "Carik gadang ngelintang asri."},
            "minang": {"kata": "hijau / ijau", "contoh": "Bukik Barisan hijau mandamai ati."},
            "bugis": {"kata": "gadong", "aksara": "ᨁᨉᨚ", "contoh": "Dau-daung gadong marica."},
            "makassar": {"kata": "moncong", "contoh": "Bulu' moncong kacci' ammolong."},
            "batak_toba": {"kata": "rata", "aksara": "ᯒᯖ", "contoh": "Rumarata hauma ni parhuta."},
            "aceh": {"kata": "ijo", "contoh": "Ija ijo seureubeng keupula."},
            "banjar": {"kata": "hijau", "contoh": "Pohon asam hijau babuah lebat."},
            "lampung": {"kata": "ijou", "aksara": "ᰀᰠᰩ", "contoh": "Kebun kopi ijou gemerlap."},
            "papua": {"kata": "hijau", "contoh": "Hutan rimba hijau lebat Papua."}
        }
    },
    {
        "indonesia": "biru", "kelas_kata": "adjektiva", "kategori": "adjektiva_warna",
        "daerah": {
            "jawa": {"ngoko": "biru", "krama_inggil": "biru", "aksara": "ꦧꦶꦫꦸ", "contoh": "Langit biru padhang tanpa mendhung."},
            "sunda": {"loma": "bulao / biru", "lemes_batur": "bulao", "aksara": "ᮘᮥᮜᮃᮇ", "contoh": "Langit bulao beresih caang."},
            "bali": {"andap": "pelkung / biru", "alus_singgih": "nila", "aksara": "ᬦᬶᬮ", "contoh": "Segara nila pelkung matumpuk."},
            "minang": {"kata": "biru", "contoh": "Danau Maninjau biru mangilap."},
            "bugis": {"kata": "gau'", "aksara": "ᨁᨕᨘ", "contoh": "Langina gau' masagena."},
            "makassar": {"kata": "kebo gau", "contoh": "Tamparang biru la'busi."},
            "batak_toba": {"kata": "rata / biru", "aksara": "ᯅᯪᯒᯮ", "contoh": "Tao Toba na balga jala rata biru."},
            "aceh": {"kata": "biru", "contoh": "Laot biru luah ban sigom dunya."},
            "banjar": {"kata": "biru", "contoh": "Kain sasirangan motif ombak biru."},
            "lampung": {"kata": "biru", "aksara": "ᰓᰤᰛᰪ", "contoh": "Teluk Lampung biru jernih."},
            "papua": {"kata": "biru", "contoh": "Air laut Raja Ampat biru toska memukau."}
        }
    },

    # 3. VERBA (KATA KERJA UTAMA)
    {
        "indonesia": "makan", "kelas_kata": "verba", "kategori": "verba_aktivitas",
        "daerah": {
            "jawa": {"ngoko": "mangan", "krama_inggil": "dhahar", "aksara": "ꦝꦲꦂ", "contoh": "Kula sampun dhahar sekul liwet."},
            "sunda": {"loma": "dahar", "lemes_batur": "tuang", "aksara": "ᮒᮥᮃᮀ", "contoh": "Mangga atuh geura tuang heula."},
            "bali": {"andap": "medaar", "alus_singgih": "ngrayunin", "aksara": "ᬗꭇᬬᬸᬦᬶᬦ", "contoh": "Ida Pedanda sampun ngrayunin."},
            "minang": {"kata": "makan", "contoh": "Makan bajamba tradisi basamo."},
            "bugis": {"kata": "manre", "aksara": "ᨆᨋᨙ", "contoh": "Engkalingako manre nanre pulo."},
            "makassar": {"kata": "ngnganre", "contoh": "Ammotere'ki' nampa ngnganre."},
            "batak_toba": {"kata": "mangan", "aksara": "ᯔᯝᯉ", "contoh": "Mangan hita dongan sabutuha."},
            "aceh": {"kata": "pajoh bu", "contoh": "Jak pajoh bu u rumoh nek."},
            "banjar": {"kata": "makan", "contoh": "Kawa makan soto banjar basusun."},
            "lampung": {"kata": "mengan", "aksara": "ᰕᰠᰅᰦᰍ", "contoh": "Mengan jejama di lamban tuha."},
            "papua": {"kata": "makan", "contoh": "Kitorang makan papeda dengan kuah kuning."}
        }
    },
    {
        "indonesia": "minum", "kelas_kata": "verba", "kategori": "verba_aktivitas",
        "daerah": {
            "jawa": {"ngoko": "ngombe", "krama_inggil": "ngunjuk", "aksara": "ꦔꦸꦚ꧀ꦗꦸꦏ", "contoh": "Bapak nembe ngunjuk wedang jahe."},
            "sunda": {"loma": "nginum", "lemes_batur": "ngaleueut", "aksara": "ᮍᮜᮩᮉᮒ", "contoh": "Mangga ngaleueut cai enteh haneut."},
            "bali": {"andap": "nginem", "alus_singgih": "ngranap", "aksara": "ᬗꭇᬦᬧ", "contoh": "Sampun raris ngranap toya ening."},
            "minang": {"kata": "minum", "contoh": "Minum teh talua manambah tanago."},
            "bugis": {"kata": "minung", "aksara": "ᨆᨗᨊᨘ", "contoh": "Minung uwae masiga'."},
            "makassar": {"kata": "nginung", "contoh": "Nginung je'ne' tassi'la'."},
            "batak_toba": {"kata": "manginum", "aksara": "ᯔᯝᯪᯉᯮᯔ", "contoh": "Manginum aek sitio-tio."},
            "aceh": {"kata": "jieb", "contoh": "Jieb kupi ulee kareng."},
            "banjar": {"kata": "nginum", "contoh": "Nginum banyu baram di pondok."},
            "lampung": {"kata": "nginum", "aksara": "ᰅᰤᰍᰪᰕ", "contoh": "Nginum kupi Lampung panas."},
            "papua": {"kata": "minum", "contoh": "Minum air kelapa muda tepi pantai."}
        }
    },
    {
        "indonesia": "tidur", "kelas_kata": "verba", "kategori": "verba_aktivitas",
        "daerah": {
            "jawa": {"ngoko": "turu", "krama_inggil": "sare", "aksara": "ꦱꦫꦺ", "contoh": "Eyang taksih sare wonten kamar."},
            "sunda": {"loma": "sare", "lemes_batur": "kulem", "aksara": "ᮊᮥᮜᮨᮙ", "contoh": "Pun bapa nuju kulem tibra."},
            "bali": {"andap": "pules", "alus_singgih": "makolem", "aksara": "ᬫᬓᭀᬮᬾᬫ", "contoh": "Ida Pedanda makolem ring gria."},
            "minang": {"kata": "lalok", "contoh": "Lalok lapeh di tangah surau."},
            "bugis": {"kata": "matinro", "aksara": "ᨆᨈᨗᨋᨚ", "contoh": "Matinro masiga ri ambo'na."},
            "makassar": {"kata": "tinro", "contoh": "Mange tinro ri balla' lakbusi."},
            "batak_toba": {"kata": "modom", "aksara": "ᯔᯬᯑᯬᯔ", "contoh": "Modom ma ho anggia nasonang."},
            "aceh": {"kata": "eh", "contoh": "Eh beuglah sampoe beungoh."},
            "banjar": {"kata": "guring", "contoh": "Guring nyenyak mandangar hujan."},
            "lampung": {"kata": "pedom", "aksara": "ᰎᰦᰌᰩᰕ", "contoh": "Pedom di lamban handak."},
            "papua": {"kata": "tidur", "contoh": "Tidur di honai dekat perapian hangat."}
        }
    },
    {
        "indonesia": "mandi", "kelas_kata": "verba", "kategori": "verba_aktivitas",
        "daerah": {
            "jawa": {"ngoko": "adus", "krama_inggil": "siram", "aksara": "ꦱꦶꦫꦩ", "contoh": "Bapak sampun siram toya anget."},
            "sunda": {"loma": "mandi", "lemes_batur": "ibak", "aksara": "ᮄᮘᮊ", "contoh": "Ibu nuju ibak di pancuran."},
            "bali": {"andap": "manjus", "alus_singgih": "masiram", "aksara": "ᬫᬲᬶᬭᬫ", "contoh": "Ida masiram ring beji suci."},
            "minang": {"kata": "mandi", "contoh": "Mandi balimau manyambuik puaso."},
            "bugis": {"kata": "cemme", "aksara": "ᨌᨛᨆᨙ", "contoh": "Cemme uwae kaliki mabassa."},
            "makassar": {"kata": "jemme", "contoh": "A'jemme' ri binanga bira."},
            "batak_toba": {"kata": "maridi", "aksara": "ᯔᯒᯪᯑᯪ", "contoh": "Maridi di aek sampuran."},
            "aceh": {"kata": "mano", "contoh": "Mano ie laot meuseunang."},
            "banjar": {"kata": "mandi", "contoh": "Mandi di batang banyu Martapura."},
            "lampung": {"kata": "mandui", "aksara": "ᰕᰦᰍᰌᰪᰤ", "contoh": "Mandui di way jernih seger."},
            "papua": {"kata": "mandi", "contoh": "Mandi di kali bening pegunungan."}
        }
    },
    {
        "indonesia": "bangun", "kelas_kata": "verba", "kategori": "verba_aktivitas",
        "daerah": {
            "jawa": {"ngoko": "tangi", "krama_inggil": "wungu", "aksara": "ꦮꦸꦔꦸ", "contoh": "Wungu enjang lajeng sembahyang."},
            "sunda": {"loma": "hudang", "lemes_batur": "gugah", "aksara": "ᮌᮥᮌᮂ", "contoh": "Gugah enjing-enjing ngabresihan buruan."},
            "bali": {"andap": "bangun", "alus_singgih": "matangi", "aksara": "ᬫᬢꬌᬶ", "contoh": "Matangi semeng ngaturang sembah."},
            "minang": {"kata": "jago / bangun", "contoh": "Jago subuah manjalang fajar."},
            "bugis": {"kata": "moto", "aksara": "ᨆᨚᨈᨚ", "contoh": "Moto ele' maccule."},
            "makassar": {"kata": "ambangung", "contoh": "Ambangung subu asambayang."},
            "batak_toba": {"kata": "dungo", "aksara": "ᯑᯮᯝᯬ", "contoh": "Dungo sian podoman sogot."},
            "aceh": {"kata": "beudoh", "contoh": "Beudoh beungoh sembahyang suboh."},
            "banjar": {"kata": "bangun", "contoh": "Bangun baisukan mandangar bang."},
            "lampung": {"kata": "megegah", "aksara": "ᰕᰦᰎᰦᰎᰦᰝ", "contoh": "Megegah debingi ngisap embun."},
            "papua": {"kata": "bangun", "contoh": "Bangun pagi dengar kicau burung kasuari."}
        }
    },
    {
        "indonesia": "berjalan", "kelas_kata": "verba", "kategori": "verba_pergerakan",
        "daerah": {
            "jawa": {"ngoko": "mlaku", "krama_inggil": "tindak", "aksara": "ꦠꦶꦤ꧀ꦢꦏ", "contoh": "Kula tindak dhateng peken."},
            "sunda": {"loma": "leumpang", "lemes_batur": "mapah", "aksara": "ᮙᮕᮂ", "contoh": "Mapah sasarengan ka sakola."},
            "bali": {"andap": "majalan", "alus_singgih": "mamargi", "aksara": "ᬫᬫꭇᬕᬶ", "contoh": "Mamargi nuju pura agung."},
            "minang": {"kata": "bajalan", "contoh": "Bajalan dialuih tantang padang."},
            "bugis": {"kata": "jokka", "aksara": "ᨍᨚᨀ", "contoh": "Jokka-jokka ri kota makessing."},
            "makassar": {"kata": "jappa", "contoh": "Jappa-jappa ri Pantai Losari."},
            "batak_toba": {"kata": "mardalan", "aksara": "ᯔᯒ᯲ᯑᯞᯉ", "contoh": "Mardalan tu bona pasogit."},
            "aceh": {"kata": "jak", "contoh": "Jak beuklueh bak bineh pante."},
            "banjar": {"kata": "bajalanan", "contoh": "Bajalanan santai ka pasar terapung."},
            "lampung": {"kata": "lapah", "aksara": "ᰜᰦᰎᰦᰝ", "contoh": "Lapah jejama sekula."},
            "papua": {"kata": "jalan", "contoh": "Jalan kaki melintasi hutan bakau."}
        }
    },
    {
        "indonesia": "berlari", "kelas_kata": "verba", "kategori": "verba_pergerakan",
        "daerah": {
            "jawa": {"ngoko": "mlayu", "krama_inggil": "lumayu", "aksara": "ꦭꦸꦩꦪꦸ", "contoh": "Lare-lare lumayu wonten plataran."},
            "sunda": {"loma": "lumpat", "lemes_batur": "lumpat", "aksara": "ᮜᮥᮙ᮪ᮕᮒ", "contoh": "Budak lumpat moro langlayangan."},
            "bali": {"andap": "malaib", "alus_singgih": "malaib", "aksara": "ᬫᬮᬶᬩ", "contoh": "Malaib digelis nuju genah aman."},
            "minang": {"kata": "maluncua / balari", "contoh": "Balari kancang ka tapi aia."},
            "bugis": {"kata": "lumpa / lari", "aksara": "ᨒᨘᨄ", "contoh": "Lumpa massiga ri tana lapang."},
            "makassar": {"kata": "lari", "contoh": "A'lari kence' ampa'jari juara."},
            "batak_toba": {"kata": "marlojong", "aksara": "ᯔᯒ᯲ᯞᯬᯐᯬᯰ", "contoh": "Marlojong tu balian manjumpang."},
            "aceh": {"kata": "plueng", "contoh": "Plueng bagah-bagah beu trép."},
            "banjar": {"kata": "bukah", "contoh": "Bukah takutan diuber anjing."},
            "lampung": {"kata": "pelay", "aksara": "ᰎᰦᰜᰦᰤ", "contoh": "Pelay gegas sampai lamban."},
            "papua": {"kata": "lari", "contoh": "Lari kencang kejar bola di lapangan."}
        }
    },
    {
        "indonesia": "duduk", "kelas_kata": "verba", "kategori": "verba_posisi",
        "daerah": {
            "jawa": {"ngoko": "lungguh", "krama_inggil": "lenggah", "aksara": "ꦭꦺꦁꦒꦃ", "contoh": "Kula aturi lenggah wonten pendhapi."},
            "sunda": {"loma": "diuk", "lemes_batur": "calik", "aksara": "ᮎᮜᮤᮊ", "contoh": "Mangga calik dina korsi goyang."},
            "bali": {"andap": "negak", "alus_singgih": "malinggih", "aksara": "ᬫᬮᬶꬌᬶᬄ", "contoh": "Titiang malinggih ring bale banjar."},
            "minang": {"kata": "duduak", "contoh": "Duduak baselo mananti hidangan."},
            "bugis": {"kata": "tudang", "aksara": "ᨈᨘᨉ", "contoh": "Tudang sipulung musyawarah mupakat."},
            "makassar": {"kata": "mempo", "contoh": "Mempo ammenteng ri baruga."},
            "batak_toba": {"kata": "hundul", "aksara": "ᯂᯮᯉ᯲ᯑᯮᯞ", "contoh": "Hundul di amak tiar adat."},
            "aceh": {"kata": "doe'", "contoh": "Doe' meusapat peugot keureuja."},
            "banjar": {"kata": "duduk", "contoh": "Duduk basila mandangar caramah."},
            "lampung": {"kata": "mejong", "aksara": "ᰕᰦᰗᰩᰅ", "contoh": "Mejong di beranda mengan jejama."},
            "papua": {"kata": "duduk", "contoh": "Duduk melingkar cerita dongeng leluhur."}
        }
    },
    {
        "indonesia": "berdiri", "kelas_kata": "verba", "kategori": "verba_posisi",
        "daerah": {
            "jawa": {"ngoko": "ngadeg", "krama_inggil": "jumeneng", "aksara": "ꦗꦸꦩꦼꦤꦼꦁ", "contoh": "Kanjeng Sultan jumeneng dados nata."},
            "sunda": {"loma": "nangtung", "lemes_batur": "ngadeg", "aksara": "ᮍᮓᮨᮌ", "contoh": "Ngadeg sing lempeng hormat bandéra."},
            "bali": {"andap": "ngadeg", "alus_singgih": "ngadeg", "aksara": "ᬗᬤᬾᬕ", "contoh": "Ngadeg jejeg ring ajeng candi."},
            "minang": {"kata": "tagak", "contoh": "Tagak indak bakisa lambang kateguhan."},
            "bugis": {"kata": "tetdong", "aksara": "ᨈᨛᨈᨚ", "contoh": "Tetdong mappasilasa ada."},
            "makassar": {"kata": "ammenteng", "contoh": "Ammenteng kokoh ri butta."},
            "batak_toba": {"kata": "jongjong", "aksara": "ᯐᯬᯰᯐᯬᯰ", "contoh": "Jongjong hita di jolo ni pargompulon."},
            "aceh": {"kata": "dong", "contoh": "Dong beutot teuntee bak janji."},
            "banjar": {"kata": "badiri", "contoh": "Badiri tagap manatap sungai."},
            "lampung": {"kata": "tejep", "aksara": "ᰊᰦᰗᰦᰎ", "contoh": "Tejep teges mimpin pekon."},
            "papua": {"kata": "berdiri", "contoh": "Berdiri tegak pandang lautan luas."}
        }
    },
    {
        "indonesia": "melihat", "kelas_kata": "verba", "kategori": "verba_pancaindra",
        "daerah": {
            "jawa": {"ngoko": "ndeleng", "krama_inggil": "mirsani", "aksara": "ꦩꦶꦂꦱꦤꦶ", "contoh": "Bapak mirsani pagelaran wayang kulit."},
            "sunda": {"loma": "nempo", "lemes_batur": "ningali", "aksara": "ᮔᮤ8ᮜᮤ", "contoh": "Ningali pamandangan gunung asri pisan."},
            "bali": {"andap": "ningalin", "alus_singgih": "ngaksi", "aksara": "ᬗᬓᬲᬶ", "contoh": "Ngaksi tarian sakral ring pura."},
            "minang": {"kata": "mancaliak", "contoh": "Mancaliak panorama Danau Singkarak."},
            "bugis": {"kata": "mita", "aksara": "ᨆᨗᨈ", "contoh": "Mita kessingna tana Bone."},
            "makassar": {"kata": "accini'", "contoh": "Accini' bulaeng makelle'."},
            "batak_toba": {"kata": "mangida", "aksara": "ᯔᯝᯪᯑ", "contoh": "Mangida uli ni Tao Toba."},
            "aceh": {"kata": "ngieng", "contoh": "Ngieng ceudah ceulatuk meukutob."},
            "banjar": {"kata": "malihat", "contoh": "Malihat kembang melati babungaan."},
            "lampung": {"kata": "ngeliak", "aksara": "ᰅᰦᰜᰤᰦᰀ", "contoh": "Ngeliak gajah di Way Kambas."},
            "papua": {"kata": "lia", "contoh": "Lia burung cenderawasih menari."}
        }
    },
    {
        "indonesia": "mendengar", "kelas_kata": "verba", "kategori": "verba_pancaindra",
        "daerah": {
            "jawa": {"ngoko": "ngrungu", "krama_inggil": "midhanget", "aksara": "ꦩꦶꦝꦔꦼꦠ", "contoh": "Midhanget swanten gamelan laras slendro."},
            "sunda": {"loma": "ngadenge", "lemes_batur": "ngadangu", "aksara": "ᮍᮓ8ᮥ", "contoh": "Ngadangu tembang Cianjuran halimpu."},
            "bali": {"andap": "ningeh", "alus_singgih": "mireng", "aksara": "ᬫᬶꭇᬗ", "contoh": "Mirengang suara genta pedanda."},
            "minang": {"kata": "mandanga", "contoh": "Mandanga saluang babunyi sayu."},
            "bugis": {"kata": "marengkalinge", "aksara": "ᨆᨑᨛᨀᨒᨗᨂᨙ", "contoh": "Marengkalinge paseng to riolo."},
            "makassar": {"kata": "ma'langngere'", "contoh": "Ma'langngere' ada pappasang baji'."},
            "batak_toba": {"kata": "manangihon", "aksara": "ᯔᯉꯥᯝᯪᯂᯬᯉ", "contoh": "Manangihon poda ni natoras."},
            "aceh": {"kata": "deungo", "contoh": "Deungo seumapa meukalam."},
            "banjar": {"kata": "mandangar", "contoh": "Mandangar habar sanang matan dangsanak."},
            "lampung": {"kata": "ngedengis", "aksara": "ᰅᰦᰌᰦᰅᰤᰠ", "contoh": "Ngedengis cawa tuha bijak."},
            "papua": {"kata": "dengar", "contoh": "Dengar suara tifa di kampung adat."}
        }
    },
    {
        "indonesia": "berbicara", "kelas_kata": "verba", "kategori": "verba_komunikasi",
        "daerah": {
            "jawa": {"ngoko": "omong", "krama_inggil": "ngendika", "aksara": "ꦔꦼꦤ꧀ꦢꦶꦏ", "contoh": "Kanjeng rama ngendika bab kautaman budi."},
            "sunda": {"loma": "ngomong", "lemes_batur": "nyarios", "aksara": "8ᮃᮛᮤᮇᮞ", "contoh": "Nyarios basa Sunda lemes ka saluhureun."},
            "bali": {"andap": "ngomong", "alus_singgih": "mabaos", "aksara": "ᬫᬩᬑᬲ", "contoh": "Mabaos alus singgih ring pasamuan."},
            "minang": {"kata": "mangecek", "contoh": "Mangecek lunak mananua kawan."},
            "bugis": {"kata": "mabbicara", "aksara": "ᨆᨅᨗᨌᨑ", "contoh": "Mabbicara ada adeq lempu."},
            "makassar": {"kata": "abbicara", "contoh": "Abbicara sirina tau baji'."},
            "batak_toba": {"kata": "mangkatai", "aksara": "ᯔᯰᯂᯖᯤ", "contoh": "Mangkatai denggan di tonga ni huta."},
            "aceh": {"kata": "meututoe", "contoh": "Meututoe ngon sopan santon."},
            "banjar": {"kata": "bapandir", "contoh": "Bapandir langkar manambahi akrab."},
            "lampung": {"kata": "cawa", "aksara": "ᰅᰦᰓᰦ", "contoh": "Cawa helau bujurai rasa."},
            "papua": {"kata": "bicara", "contoh": "Bicara sopan dengan tetua adat."}
        }
    },
    {
        "indonesia": "membaca", "kelas_kata": "verba", "kategori": "verba_pendidikan",
        "daerah": {
            "jawa": {"ngoko": "maca", "krama_inggil": "maos", "aksara": "ꦩꦲꦺꦴꦱ", "contoh": "Maos serat Primbon peninggalan leluhur."},
            "sunda": {"loma": "maca", "lemes_batur": "maos", "aksara": "ᮙᮇᮞ", "contoh": "Maos carita pantun Sunda Buhun."},
            "bali": {"andap": "maca", "alus_singgih": "ngwacen", "aksara": "ᬗꭇᬯᬘᬾᬦ", "contoh": "Ngwacen lontar kakawin Arjuna Wiwaha."},
            "minang": {"kata": "mambaco", "contoh": "Mambaco tambo adaik Minangkabau."},
            "bugis": {"kata": "mabbaca", "aksara": "ᨆᨅᨌ", "contoh": "Mabbaca Sureq I La Galigo."},
            "makassar": {"kata": "abbaca", "contoh": "Abbaca lontara' bilang."},
            "batak_toba": {"kata": "manjaha", "aksara": "ᯔᯉꯥᯐᯂ", "contoh": "Manjaha pustaha laklak Batak."},
            "aceh": {"kata": "baca", "contoh": "Baca hikayat Prang Sabi."},
            "banjar": {"kata": "mambaca", "contoh": "Mambaca syair Maulid Habsyi."},
            "lampung": {"kata": "ngebaca", "aksara": "ᰅᰦᰓᰦᰅᰦ", "contoh": "Ngebaca piil pesenggiri."},
            "papua": {"kata": "baca", "contoh": "Baca buku sejarah tanah Papua."}
        }
    },
    {
        "indonesia": "menulis", "kelas_kata": "verba", "kategori": "verba_pendidikan",
        "daerah": {
            "jawa": {"ngoko": "nulis", "krama_inggil": "nyerat", "aksara": "ꦚꦼꦫꦠ", "contoh": "Nyerat piwulang ngangge aksara Hanacaraka."},
            "sunda": {"loma": "nulis", "lemes_batur": "nyerat", "aksara": "8ᮨᮛᮒ", "contoh": "Nyerat sisindiran ku aksara Sunda."},
            "bali": {"andap": "nulis", "alus_singgih": "nyurat", "aksara": "ᬜᬸꭇᬢ", "contoh": "Nyurat lontar nganggo pengrupak."},
            "minang": {"kata": "manulih", "contoh": "Manulih pantun parambahan Minang."},
            "bugis": {"kata": "manulis", "aksara": "ᨆᨊᨘᨒᨗ", "contoh": "Manulis aksara Lontara Ogi."},
            "makassar": {"kata": "anulis", "contoh": "Anulis sura' battu ri Bone."},
            "batak_toba": {"kata": "manurat", "aksara": "ᯔᯉꯥᯮᯒᯖ", "contoh": "Manurat Surat Batak di pahu."},
            "aceh": {"kata": "tuleh", "contoh": "Tuleh haba peuingat aneuk nanggroe."},
            "banjar": {"kata": "manulis", "contoh": "Manulis riwayat ulama Martapura."},
            "lampung": {"kata": "nulis", "aksara": "ᰍᰪᰜᰤᰠ", "contoh": "Nulis Surat Lampung Kaganga."},
            "papua": {"kata": "tulis", "contoh": "Tulis kisah ukiran Asmat yang megah."}
        }
    },
    {
        "indonesia": "bekerja", "kelas_kata": "verba", "kategori": "verba_ekonomi",
        "daerah": {
            "jawa": {"ngoko": "nyambut gawe", "krama_inggil": "ngasta karya", "aksara": "ꦔꦱ꧀ꦠ ꦏꦂꦪ", "contoh": "Ngasta karya kanthi temen lan ikhlas."},
            "sunda": {"loma": "gawe", "lemes_batur": "didamel", "aksara": "ᮓᮤᮓᮙᮨᮜ", "contoh": "Didamel di pabrik satia tur geten."},
            "bali": {"andap": "ngae gawe", "alus_singgih": "makarya", "aksara": "ᬫꭇꬓᬭᬬ", "contoh": "Makarya sinarengan ring carik."},
            "minang": {"kata": "karajo", "contoh": "Karajo bajanjang naiak batangga turun."},
            "bugis": {"kata": "jama", "aksara": "ᨍᨆ", "contoh": "Jama makessing ri dare'na."},
            "makassar": {"kata": "a'jama", "contoh": "A'jama pabrik la'busi."},
            "batak_toba": {"kata": "karejo", "aksara": "ᯂᯒᯩᯐᯬ", "contoh": "Makkarejo di hauma sabaraha ari."},
            "aceh": {"kata": "mubuet", "contoh": "Mubuet beukeuma get meuhase."},
            "banjar": {"kata": "bagawi", "contoh": "Bagawi bujur gasan nafkah kaluarga."},
            "lampung": {"kata": "begawi", "aksara": "ᰓᰦᰎᰦᰓᰤ", "contoh": "Begawi gawi adat pepadun."},
            "papua": {"kata": "kerja", "contoh": "Kerja kebun ubi ramai-ramai."}
        }
    },
    {
        "indonesia": "belajar", "kelas_kata": "verba", "kategori": "verba_pendidikan",
        "daerah": {
            "jawa": {"ngoko": "sinau", "krama_inggil": "nyinau", "aksara": "ꦱꦶꦤꦲꦸ", "contoh": "Sinau basa daerah murih lestari."},
            "sunda": {"loma": "diajar", "lemes_batur": "diajar", "aksara": "ᮓᮤᮃᮏᮁ", "contoh": "Diajar basa Sunda di sakolaan."},
            "bali": {"andap": "mlajah", "alus_singgih": "malajah", "aksara": "ᬫᬮᬚᬄ", "contoh": "Malajah ngigel Bali ring sanggar."},
            "minang": {"kata": "baraja", "contoh": "Baraja basilek di sasaran adaik."},
            "bugis": {"kata": "magguru", "aksara": "ᨆᨁᨘᨑᨘ", "contoh": "Magguru mabbicara Ogi."},
            "makassar": {"kata": "a'guru", "contoh": "A'guru battu ri pajjama matoa."},
            "batak_toba": {"kata": "marsiajar", "aksara": "ᯔᯒ᯲ᯘᯪᯀᯐᯒ", "contoh": "Marsiajar adat Batak asa malo."},
            "aceh": {"kata": "beuet", "contoh": "Beuet kitab bak dayah ulama."},
            "banjar": {"kata": "balajar", "contoh": "Balajar maulah wadai tradisional."},
            "lampung": {"kata": "belajagh", "aksara": "ᰓᰦᰜᰦᰗᰦᰛ", "contoh": "Belajagh sastra Lampung klasik."},
            "papua": {"kata": "belajar", "contoh": "Belajar anyam noken dari mama."}
        }
    },
    {
        "indonesia": "membeli", "kelas_kata": "verba", "kategori": "verba_perniagaan",
        "daerah": {
            "jawa": {"ngoko": "tuku", "krama_inggil": "mundhut", "aksara": "ꦩꦸꦤ꧀ꦝꦸꦠ", "contoh": "Ibu mundhut bathik tulis Solo."},
            "sunda": {"loma": "meuli", "lemes_batur": "ngagaleuh", "aksara": "ᮍᮌᮜᮩᮂ", "contoh": "Ngagaleuh beas Cianjur di pasar."},
            "bali": {"andap": "meli", "alus_singgih": "matumbasan", "aksara": "ᬫᬢᬸᬫꭇᬩᬲᬦ", "contoh": "Matumbasan sarana upakara suci."},
            "minang": {"kata": "mambali", "contoh": "Mambali songket Pandai Sikek."},
            "bugis": {"kata": "melli", "aksara": "ᨆᨛᨒᨗ", "contoh": "Melli lipa' sabbe Sengkang."},
            "makassar": {"kata": "ammalli", "contoh": "Ammalli juku' ri pasar Paotere."},
            "batak_toba": {"kata": "manuhor", "aksara": "ᯔᯉꯥᯮᯂᯬᯒ", "contoh": "Manuhor ulos sadum di onan."},
            "aceh": {"kata": "bloe", "contoh": "Bloe ija silweueng aceh."},
            "banjar": {"kata": "manukar", "contoh": "Manukar wadai bingka barandam."},
            "lampung": {"kata": "ngebeli", "aksara": "ᰅᰦᰓᰦᰜᰤ", "contoh": "Ngebeli seruit ikan baung."},
            "papua": {"kata": "beli", "contoh": "Beli noken kulit kayu di pasar Hamadi."}
        }
    },
    {
        "indonesia": "menjual", "kelas_kata": "verba", "kategori": "verba_perniagaan",
        "daerah": {
            "jawa": {"ngoko": "ngadol", "krama_inggil": "nyade", "aksara": "ꦚꦢꦺ", "contoh": "Nyade asil panen pari ing kitha."},
            "sunda": {"loma": "ngajual", "lemes_batur": "ngical", "aksara": "8ᮤᮎᮜ", "contoh": "Ngical bubuahan seger di sisi jalan."},
            "bali": {"andap": "ngadol", "alus_singgih": "ngadol", "aksara": "ᬗᬤᭀᬮ", "contoh": "Ngadol woh-wohan ring pasar Kumbasari."},
            "minang": {"kata": "manjua", "contoh": "Manjua lado merah Bukittinggi."},
            "bugis": {"kata": "mabaluk", "aksara": "ᨆᨅᨒᨘ", "contoh": "Mabaluk berre' ri pasar Sentral."},
            "makassar": {"kata": "ambaluk", "contoh": "Ambaluk juku' cakalang."},
            "batak_toba": {"kata": "manggadis", "aksara": "ᯔꯥᯰᯎᯑᯪᯘ", "contoh": "Manggadis kopi Sidikalang di balige."},
            "aceh": {"kata": "publoe", "contoh": "Publoe kupi gayo harum."},
            "banjar": {"kata": "majual", "contoh": "Majual iwak karing sapat di jukung."},
            "lampung": {"kata": "ngejual", "aksara": "ᰅᰦᰗᰪᰓᰦᰜ", "contoh": "Ngejual lada hitam ekspor."},
            "papua": {"kata": "jual", "contoh": "Jual hasil kebun dan ikan segar di pasar."}
        }
    },
    {
        "indonesia": "memasak", "kelas_kata": "verba", "kategori": "verba_aktivitas",
        "daerah": {
            "jawa": {"ngoko": "masak", "krama_inggil": "olah-olah", "aksara": "ꦲꦺꦴꦭꦃꦲꦺꦴꦭꦃ", "contoh": "Ibu olah-olah gudeg manggar klangenan."},
            "sunda": {"loma": "masak", "lemes_batur": "ngaliwet", "aksara": "ᮍᮜᮤᮝᮨᮒ", "contoh": "Ngaliwet kastrol pulen sareng peda."},
            "bali": {"andap": "ngolah", "alus_singgih": "meratengan", "aksara": "ᬫꭇꬓᬢꬌᬦ", "contoh": "Meratengan lawar kuwir becik pisan."},
            "minang": {"kata": "mamasak", "contoh": "Mamasak randang dagiang nan legit."},
            "bugis": {"kata": "mannasu", "aksara": "ᨆᨊᨔᨘ", "contoh": "Mannasu nasu palekko pedas."},
            "makassar": {"kata": "anasu", "contoh": "Anasu coto makassar harum."},
            "batak_toba": {"kata": "mangalompa", "aksara": "ᯔꯥᯝᯞᯬᯔ᯲ᯇ", "contoh": "Mangalompa arsik dengke mas."},
            "aceh": {"kata": "meutaguen", "contoh": "Meutaguen kuah beulangong gampong."},
            "banjar": {"kata": "mamasak", "contoh": "Mamasak gangan asam patin sedap."},
            "lampung": {"kata": "ngemasak", "aksara": "ᰅᰦᰕᰦᰠᰦᰀ", "contoh": "Ngemasak seruit sambal rampai."},
            "papua": {"kata": "masak", "contoh": "Masak ubi batu panas dalam upacara."}
        }
    },
    {
        "indonesia": "mencuci", "kelas_kata": "verba", "kategori": "verba_aktivitas",
        "daerah": {
            "jawa": {"ngoko": "ngumbah", "krama_inggil": "nggirah", "aksara": "ꦔ꧀ꦒꦶꦫꦃ", "contoh": "Nggirah busana bathik sutra."},
            "sunda": {"loma": "nyeuseuh", "lemes_batur": "ngumbah", "aksara": "8ᮩᮞᮩᮂ", "contoh": "Nyeuseuh acuk di pancuran cai."},
            "bali": {"andap": "ngumbah", "alus_singgih": "masuh", "aksara": "ᬫᬲᬸᬄ", "contoh": "Masuh wastra sembahyang resik."},
            "minang": {"kata": "mambasuah", "contoh": "Mambasuah kaki sabalun masuak rumah."},
            "bugis": {"kata": "mabbissa", "aksara": "ᨆᨅᨗᨔ", "contoh": "Mabbissa baju lipa' sabbe."},
            "makassar": {"kata": "abbissa", "contoh": "Abbissa jaroe riolo ngnganre."},
            "batak_toba": {"kata": "mamuri", "aksara": "ᯔᯔᯮᯒᯪ", "contoh": "Mamuri tangan di aek sitio."},
            "aceh": {"kata": "rhah", "contoh": "Rhah bajee ngon ie krueng."},
            "banjar": {"kata": "mambasuh", "contoh": "Mambasuh tangan sabalum mamamah wadai."},
            "lampung": {"kata": "mebasuh", "aksara": "ᰕᰦᰓᰦᰠᰪᰝ", "contoh": "Mebasuh pungu sai bresih."},
            "papua": {"kata": "cuci", "contoh": "Cuci noken di kali air jernih."}
        }
    },
    {
        "indonesia": "tertawa", "kelas_kata": "verba", "kategori": "verba_ekspresi",
        "daerah": {
            "jawa": {"ngoko": "ngguyu", "krama_inggil": "gumujeng", "aksara": "ꦒꦸꦩꦸꦗꦼꦁ", "contoh": "Para tamu gumujeng mireng guyonan dagelan."},
            "sunda": {"loma": "seuri", "lemes_batur": "gumujeng", "aksara": "ᮌᮥᮙᮥᮏᮨᮀ", "contoh": "Gumujeng bungah ngadangu lulucon bobodoran."},
            "bali": {"andap": "kedek", "alus_singgih": "kenyem", "aksara": "ᬓᬾᬜᬾᬫ", "contoh": "Kenyem manis nyanggra para tamiu."},
            "minang": {"kata": "galak", "contoh": "Galak basamo mangurangi raso panek."},
            "bugis": {"kata": "macawa", "aksara": "ᨆᨌᨓ", "contoh": "Macawa marennu ri alena."},
            "makassar": {"kata": "a'cawa", "contoh": "A'cawa rannu sari'battang."},
            "batak_toba": {"kata": "mengkel", "aksara": "ᯔᯩ꯰ᯂᯩᯞ", "contoh": "Mengkel suping mangalusi dongan."},
            "aceh": {"kata": "khem", "contoh": "Khem meuseunang gadoh duka."},
            "banjar": {"kata": "tatawa", "contoh": "Tatawa takangkang mandangar lawakan."},
            "lampung": {"kata": "mewawa", "aksara": "ᰕᰦᰓᰦᰓᰦ", "contoh": "Mewawa riang buhimpun tuha."},
            "papua": {"kata": "tertawa", "contoh": "Tertawa gembira menyambut kawan lama."}
        }
    },
    {
        "indonesia": "menangis", "kelas_kata": "verba", "kategori": "verba_ekspresi",
        "daerah": {
            "jawa": {"ngoko": "nangis", "krama_inggil": "muwun", "aksara": "ꦩꦸꦮꦸꦤ", "contoh": "Muwun amargi kapisah kaliyan kulawarga."},
            "sunda": {"loma": "ceurik", "lemes_batur": "nangis", "aksara": "ᮔ8ᮤᮞ", "contoh": "Nangis sedih ditinggal ku pupuhu."},
            "bali": {"andap": "ngeling", "alus_singgih": "ngeling", "aksara": "ᬗᬮᬶᬗ", "contoh": "Ngeling eling ring lelampahan urip."},
            "minang": {"kata": "manangih", "contoh": "Manangih ibo mancaliak mande marantau."},
            "bugis": {"kata": "teri", "aksara": "ᨈᨛᨑᨗ", "contoh": "Teri masse' ri atinna."},
            "makassar": {"kata": "a'teri", "contoh": "A'teri siri' ampa'jari lussa."},
            "batak_toba": {"kata": "tumangis", "aksara": "ᯖᯮᯔꯥᯰᯪᯘ", "contoh": "Tumangis mangandungi amang na burju."},
            "aceh": {"kata": "moe", "contoh": "Moe teu'ingat gampong haleman."},
            "banjar": {"kata": "manangis", "contoh": "Manangis marista di sudut tilam."},
            "lampung": {"kata": "miwang", "aksara": "ᰕᰤᰓᰦᰅ", "contoh": "Miwang sedih tian lapah jaoh."},
            "papua": {"kata": "menangis", "contoh": "Menangis haru saat anak kembali dari rantau."}
        }
    },
    {
        "indonesia": "bernyanyi", "kelas_kata": "verba", "kategori": "verba_seni",
        "daerah": {
            "jawa": {"ngoko": "nembang", "krama_inggil": "nyindhen", "aksara": "ꦚꦶꦤ꧀ꦝꦺꦤ", "contoh": "Nyindhen tembang Asmaradana kanthi laras."},
            "sunda": {"loma": "ngawih", "lemes_batur": "nembang", "aksara": "ᮔᮨᮙ᮪ᮘᮀ", "contoh": "Nembang pupuh Kinanti merdu pisan."},
            "bali": {"andap": "magending", "alus_singgih": "makidung", "aksara": "ᬫꭇᬓᬶᬤᬸᬗ", "contoh": "Makidung Wargasari ring upacara piodalan."},
            "minang": {"kata": "banyanyi", "contoh": "Banyanyi dendang Minang ba-saluang."},
            "bugis": {"kata": "makkacaping", "aksara": "ᨆᨀᨌᨄᨗ", "contoh": "Makkacaping kelong to Ogi."},
            "makassar": {"kata": "kelong", "contoh": "A'kelong royong pakkiok."},
            "batak_toba": {"kata": "marende", "aksara": "ᯔᯒᯩꯉ᯲ᯑᯩ", "contoh": "Marende ende Batak O Tano Batak."},
            "aceh": {"kata": "meulagu", "contoh": "Meulagu ratoh jaroe meukeumang."},
            "banjar": {"kata": "banyanyi", "contoh": "Banyanyi lagu madihin basasirangan."},
            "lampung": {"kata": "nembang", "aksara": "ᰍᰦᰕᰓᰦᰅ", "contoh": "Nembang adi-adi sastra bertutur."},
            "papua": {"kata": "menyanyi", "contoh": "Menyanyi lagu Sajojo diiringi tifa gembira."}
        }
    },
    {
        "indonesia": "menari", "kelas_kata": "verba", "kategori": "verba_seni",
        "daerah": {
            "jawa": {"ngoko": "njoget", "krama_inggil": "mbeksa", "aksara": "ꦩ꧀ꦧꦼꦏ꧀ꦱ", "contoh": "Mbeksa Tari Bedhaya Ketawang ing kraton."},
            "sunda": {"loma": "ngigel", "lemes_batur": "ngibing", "aksara": "8ᮤᮘᮤᮀ", "contoh": "Ngibing Jaipongan rancage tur dinamik."},
            "bali": {"andap": "ngigel", "alus_singgih": "mesolah", "aksara": "ᬫᬾꭇᬲᭀᬮᬄ", "contoh": "Mesolah Tari Legong Keraton luwung pisan."},
            "minang": {"kata": "manari", "contoh": "Manari Tari Piriang di ateh kaco."},
            "bugis": {"kata": "ma'gellu", "aksara": "ᨆᨁᨛᨒᨘ", "contoh": "Ma'gellu tari Paduppa tamu agung."},
            "makassar": {"kata": "a'gellu", "contoh": "A'gellu Pakarena kipas halus."},
            "batak_toba": {"kata": "manortor", "aksara": "ᯔꯥᯉᯬᯒ᯲ᯖᯬᯒ", "contoh": "Manortor somba tu Hula-hula."},
            "aceh": {"kata": "meutari", "contoh": "Meutari Saman tepuk dada serempak."},
            "banjar": {"kata": "manari", "contoh": "Manari Tari Radap Rahayu menyambut pangeran."},
            "lampung": {"kata": "ngigel", "aksara": "ᰅᰤᰎᰦᰜ", "contoh": "Ngigel Tari Sigeh Penguten anggun."},
            "papua": {"kata": "menari", "contoh": "Menari Tari Yospan lompat riang persahabatan."}
        }
    },
    {
        "indonesia": "mencintai", "kelas_kata": "verba", "kategori": "verba_perasaan",
        "daerah": {
            "jawa": {"ngoko": "nresnani", "krama_inggil": "tresna", "aksara": "ꦠꦽꦱ꧀ꦤ", "contoh": "Tresna marang budaya luhur Nusantara."},
            "sunda": {"loma": "bogoh", "lemes_batur": "mikanyaah", "aksara": "ᮙᮤᮊ8ᮃᮃᮂ", "contoh": "Mikanyaah ka sasama manusa kalayan ikhlas."},
            "bali": {"andap": "tresna", "alus_singgih": "ngasih", "aksara": "ᬗꭇᬲᬶᬄ", "contoh": "Ngasih tresna asih ring sarwa prani."},
            "minang": {"kata": "mancintoi", "contoh": "Mancintoi adaik basandi syarak."},
            "bugis": {"kata": "mamassei", "aksara": "ᨆᨆᨔᨙ", "contoh": "Mamassei siri'na wanuwa."},
            "makassar": {"kata": "mangkasii", "contoh": "Mangkasii sari'battang samua."},
            "batak_toba": {"kata": "manghaholongi", "aksara": "ᯔꯥᯰᯂᯂᯬᯞᯬ꯰ᯪ", "contoh": "Manghaholongi dongan songon dirina."},
            "aceh": {"kata": "gaseh", "contoh": "Gaseh keu poma ngon yah dunia akherat."},
            "banjar": {"kata": "mancintai", "contoh": "Mancintai kasanian banua saumuran."},
            "lampung": {"kata": "sayang", "aksara": "ᰞᰦᰤᰦᰅ", "contoh": "Sayang sesama jurai Lampung."},
            "papua": {"kata": "sayang", "contoh": "Sayang tanah pusaka Papua penuh damai."}
        }
    },
    {
        "indonesia": "berdoa", "kelas_kata": "verba", "kategori": "verba_spiritual",
        "daerah": {
            "jawa": {"ngoko": "ndonga", "krama_inggil": "nyenyuwun", "aksara": "ꦚꦼꦚꦸꦮꦸꦤ", "contoh": "Nyenyuwun kanugrahan dhumateng Gusti Hyang Widi."},
            "sunda": {"loma": "ngadua", "lemes_batur": "neneda", "aksara": "ᮔᮨᮔᮨᮓ", "contoh": "Neneda kasalametan ka Gusti Allah nu Maha Agung."},
            "bali": {"andap": "mabhakti", "alus_singgih": "ngastiti", "aksara": "ᬗꭇᬲꭇᬢᬶꭇᬢᬶ", "contoh": "Ngastiti bhakti ring Hyang Widhi Wasa."},
            "minang": {"kata": "badoa", "contoh": "Badoa mamohon salamik kasalamatan."},
            "bugis": {"kata": "maddoa", "aksara": "ᨆᨉᨚᨕ", "contoh": "Maddoa ri Puang Allah Taala."},
            "makassar": {"kata": "a'doa", "contoh": "A'doa ri karaeng pajjama barakka'."},
            "batak_toba": {"kata": "martangiang", "aksara": "ᯔᯒ᯲ᯖꯥᯰᯪᯀꯥᯰ", "contoh": "Martangiang tu Debata asa martua ngolu."},
            "aceh": {"kata": "meudoa", "contoh": "Meudoa meulakee rezeki murah."},
            "banjar": {"kata": "badoa", "contoh": "Badoa selamat gasan nagari barakah."},
            "lampung": {"kata": "bedoa", "aksara": "ᰓᰦᰌᰩᰓᰦ", "contoh": "Bedoa neda kasejahteraan pekon."},
            "papua": {"kata": "berdoa", "contoh": "Berdoa mengucap syukur atas berkat alam."}
        }
    },
    {
        "indonesia": "membantu", "kelas_kata": "verba", "kategori": "verba_sosial",
        "daerah": {
            "jawa": {"ngoko": "ngrewangi", "krama_inggil": "mbiyantu", "aksara": "ꦩ꧀ꦧꦶꦪꦤ꧀ꦠꦸ", "contoh": "Mbiyantu tangga teparo gotong royong."},
            "sunda": {"loma": "mantuan", "lemes_batur": "ngabantosan", "aksara": "8ᮘᮔ᮪ᮒᮧᮞᮔ", "contoh": "Ngabantosan tatangga ngadamel saung."},
            "bali": {"andap": "nulungin", "alus_singgih": "ngwantu", "aksara": "ᬗꭇᬯꭇᬦꭇᬢᬸ", "contoh": "Ngwantu krama banjar ngayah di pura."},
            "minang": {"kata": "manolong", "contoh": "Manolong urang susah bari bahati lapeh."},
            "bugis": {"kata": "tulung", "aksara": "ᨈᨘᨒᨘ", "contoh": "Situlung-tulung massiddi ada."},
            "makassar": {"kata": "antulung", "contoh": "Antulung tau lussa ampa'jari baji'."},
            "batak_toba": {"kata": "mangurupi", "aksara": "ᯔꯥᯰᯮᯒᯮᯇᯪ", "contoh": "Mangurupi dongan na marsitaonon."},
            "aceh": {"kata": "tulong", "contoh": "Tulong meutulong saboh gampong."},
            "banjar": {"kata": "manulungi", "contoh": "Manulungi urang tuha ba-titi di titian."},
            "lampung": {"kata": "nulung", "aksara": "ᰍᰪᰜᰪᰅ", "contoh": "Nulung tetangga bangun lamban."},
            "papua": {"kata": "bantu", "contoh": "Bantu tetangga bersihkan kebun bersama."}
        }
    },
    {
        "indonesia": "memberi", "kelas_kata": "verba", "kategori": "verba_sosial",
        "daerah": {
            "jawa": {"ngoko": "weneh", "krama_inggil": "maringi", "aksara": "ꦩꦫꦶꦔꦶ", "contoh": "Bapak maringi pitedah ingkang leres."},
            "sunda": {"loma": "mere", "lemes_batur": "maparin", "aksara": "ᮙᮕᮛᮤᮔ", "contoh": "Maparin sedekah ka kaum dhuafa."},
            "bali": {"andap": "maang", "alus_singgih": "ngicen", "aksara": "ᬗꭇᬶᬘᬾᬦ", "contoh": "Ida Pedanda ngicen panugrahan suci."},
            "minang": {"kata": "maagiah", "contoh": "Maagiah bantuan ka fakir miskin."},
            "bugis": {"kata": "mabbere", "aksara": "ᨆᨅᨛᨑᨙ", "contoh": "Mabbere seddeke ri anana' yatim."},
            "makassar": {"kata": "assare", "contoh": "Assare pammaling-maling baji'."},
            "batak_toba": {"kata": "mangalehon", "aksara": "ᯔꯥᯰꯞᯩᯂᯬᯉ", "contoh": "Mangalehon ulos holong tu pinomparna."},
            "aceh": {"kata": "jra", "contoh": "Jra sedekah bak ureueng gasien."},
            "banjar": {"kata": "mambari", "contoh": "Mambari sasadakah lawan urang yatim."},
            "lampung": {"kata": "ngeni", "aksara": "ᰅᰦᰍᰤ", "contoh": "Ngeni nasihat di bujang gadis."},
            "papua": {"kata": "kasih", "contoh": "Kasih hasil pancingan ke tetangga kampung."}
        }
    },
    {
        "indonesia": "menerima", "kelas_kata": "verba", "kategori": "verba_sosial",
        "daerah": {
            "jawa": {"ngoko": "nampa", "krama_inggil": "nampi", "aksara": "ꦤꦩ꧀ꦥꦶ", "contoh": "Nampi paringan kanthi syukur agung."},
            "sunda": {"loma": "narima", "lemes_batur": "nampi", "aksara": "ᮔᮙ᮪ᮕᮤ", "contoh": "Nampi serat uleman pernikahan barokah."},
            "bali": {"andap": "nampi", "alus_singgih": "ngambil", "aksara": "ᬦꭇᬫꭇᬧᬶ", "contoh": "Nampi waranugraha Ida Sang Hyang Widhi."},
            "minang": {"kata": "manarimo", "contoh": "Manarimo galeh pusako turun tamurun."},
            "bugis": {"kata": "manarima", "aksara": "ᨆᨊᨑᨗᨆ", "contoh": "Manarima warekkadeng pammase."},
            "makassar": {"kata": "anarima", "contoh": "Anarima passuroang tu kalabbirang."},
            "batak_toba": {"kata": "manjalo", "aksara": "ᯔꯥᯉꯥᯐᯞᯬ", "contoh": "Manjalo pasu-pasu sian angka raja."},
            "aceh": {"kata": "teurimong", "contoh": "Teurimong amanah nanggroe luhur."},
            "banjar": {"kata": "manarima", "contoh": "Manarima kado matan bubuhan karabat."},
            "lampung": {"kata": "nrimo", "aksara": "ᰍᰤᰛᰤᰕᰩ", "contoh": "Nrimo tamu di sesat agung."},
            "papua": {"kata": "trima", "contoh": "Trima tanda kehormatan mahkota kasuari."}
        }
    },

    # 4. NOMINA ANATOMI & TUBUH
    {
        "indonesia": "kepala", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "endhas", "krama_inggil": "mustaka", "aksara": "ꦩꦸꦱ꧀ꦠꦏ", "contoh": "Mustaka dipunagemi blangkon gagrag Ngayogyakarta."},
            "sunda": {"loma": "hulu", "lemes_batur": "mastaka", "aksara": "ᮙᮞ᮪ᮒᮊ", "contoh": "Mastaka dianggo bendo batik Priangan."},
            "bali": {"andap": "sirah", "alus_singgih": "prabu", "aksara": "ᬧꭇᬩᬸ", "contoh": "Prabu kadagingin udeng songket putih."},
            "minang": {"kata": "kapalo", "contoh": "Deta mangabek di kapalo penghulu."},
            "bugis": {"kata": "uluk", "aksara": "ᨕᨘᨒᨘ", "contoh": "Songko' to Bone ri ulukna."},
            "makassar": {"kata": "ulu", "contoh": "Ulu nipasang passapu eja."},
            "batak_toba": {"kata": "ulu", "aksara": "ᯥᯞᯮ", "contoh": "Tali-tali di ulu ni raja."},
            "aceh": {"kata": "ulee", "contoh": "Kupiah Meukutob bak ulee pahlawan."},
            "banjar": {"kata": "kapala", "contoh": "Laung kuning di kapala galuh."},
            "lampung": {"kata": "ulu", "aksara": "ᰀᰪᰜᰪ", "contoh": "Siger emas agung di ulu muli."},
            "papua": {"kata": "kepala", "contoh": "Mahkota bulu kasuari di kepala kepala suku."}
        }
    },
    {
        "indonesia": "mata", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "mripat", "krama_inggil": "soca", "aksara": "ꦱꦺꦴꦕ", "contoh": "Soca panjenengan tajem kados mustika."},
            "sunda": {"loma": "panon", "lemes_batur": "soca", "aksara": "ᮞᮧᮎ", "contoh": "Soca mancerong ningali kaéndahan alam."},
            "bali": {"andap": "mata", "alus_singgih": "penyingakan", "aksara": "ᬧꭇᬜꬶᬗᬓᬦ", "contoh": "Penyingakan bening nyinarang welas asih."},
            "minang": {"kata": "mato", "contoh": "Mato mancaliak ka lauik lapeh."},
            "bugis": {"kata": "mata", "aksara": "ᨆᨈ", "contoh": "Mata mita cinnong tana Ogi."},
            "makassar": {"kata": "mata", "contoh": "Mata accini' bintoeng mara'."},
            "batak_toba": {"kata": "mata", "aksara": "ᯔᯖ", "contoh": "Mata ni panondang suminar."},
            "aceh": {"kata": "mata", "contoh": "Mata kalon ceudah pusaka."},
            "banjar": {"kata": "mata", "contoh": "Mata manatap pemandangan riam."},
            "lampung": {"kata": "mata", "aksara": "ᰕᰦᰊᰦ", "contoh": "Mata mandang kebun lada."},
            "papua": {"kata": "mata", "contoh": "Mata pandang Danau Sentani indah."}
        }
    },
    {
        "indonesia": "telinga", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "kuping", "krama_inggil": "talingan", "aksara": "ꦠꦭꦶꦔꦤ꧀", "contoh": "Talingan midhangetaken swanten gentha."},
            "sunda": {"loma": "ceuli", "lemes_batur": "kuring / cepil", "aksara": "ᮎᮨᮕᮤᮜ", "contoh": "Cepil ngadangu sora suling Sunda."},
            "bali": {"andap": "kuping", "alus_singgih": "karna", "aksara": "ᬓꭇꬓ", "contoh": "Karna mirengang kidung suci."},
            "minang": {"kata": "talingo", "contoh": "Talingo mandanga bunyi rabab."},
            "bugis": {"kata": "toli", "aksara": "ᨈᨚᨒᨗ", "contoh": "Toli marengkalinge ada bajik."},
            "makassar": {"kata": "toli", "contoh": "Toli ma'langngere' pappasang toa."},
            "batak_toba": {"kata": "pinggol", "aksara": "ᯇꯪᯪᯎᯬᯞ", "contoh": "Pinggol manangihon ogung sabangunan."},
            "aceh": {"kata": "geulinyueng", "contoh": "Geulinyueng deungo rapa'i geurimpheng."},
            "banjar": {"kata": "cuping", "contoh": "Cuping mandangar galundangan sungai."},
            "lampung": {"kata": "cuping", "aksara": "ᰅᰪᰎᰤᰅ", "contoh": "Cuping ngedengis gitar tunggal."},
            "papua": {"kata": "telinga", "contoh": "Telinga dengar tabuhan tifa sahut-menyahut."}
        }
    },
    {
        "indonesia": "hidung", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "irung", "krama_inggil": "grana", "aksara": "ꦒꦿꦤ", "contoh": "Grana ngambet gandaning kembang mlathi."},
            "sunda": {"loma": "irung", "lemes_batur": "pangambung", "aksara": "ᮕ8ᮙ᮪ᮘᮥᮀ", "contoh": "Pangambung ngambeu seuseuhing malati."},
            "bali": {"andap": "cunguh", "alus_singgih": "grana", "aksara": "ᬕꭇꬓ", "contoh": "Grana ngisep miik sekar cempaka."},
            "minang": {"kata": "iduang", "contoh": "Iduang mancium aroma randang kari."},
            "bugis": {"kata": "inge'", "aksara": "ᨕᨗᨂᨙ", "contoh": "Inge' ma'bawu bua makessing."},
            "makassar": {"kata": "inge'", "contoh": "Inge' ma'bawu juku' bakar."},
            "batak_toba": {"kata": "igung", "aksara": "ᯤꯎᯮ꯰", "contoh": "Igung manganggo uap ni kopi."},
            "aceh": {"kata": "idong", "contoh": "Idong cium bago keumang."},
            "banjar": {"kata": "hidung", "contoh": "Hidung ma-cium bau kasturi harum."},
            "lampung": {"kata": "ihing", "aksara": "ᰀᰤᰝᰤᰅ", "contoh": "Ihing ngambeu wangi bungong."},
            "papua": {"kata": "hidung", "contoh": "Hidung cium wangi hutan cemara pegunungan."}
        }
    },
    {
        "indonesia": "mulut", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "cangkem", "krama_inggil": "tutuk", "aksara": "ꦠꦸꦠꦸꦏ", "contoh": "Tutuk matur ingkang sae lan santun."},
            "sunda": {"loma": "sungut", "lemes_batur": "lambey", "aksara": "ᮜᮙ᮪ᮘᮦᮚ", "contoh": "Lambey imut ngabageakeun tamu."},
            "bali": {"andap": "bibih", "alus_singgih": "tutur", "aksara": "ᬢꭇᬢꭇᬭ", "contoh": "Tutur mabaos rahayu sinarengan."},
            "minang": {"kata": "muluik", "contoh": "Muluik manis kucandan kawan."},
            "bugis": {"kata": "timu", "aksara": "ᨈᨗᨆᨘ", "contoh": "Timu mabbicara sitinaja."},
            "makassar": {"kata": "bawa", "contoh": "Bawa abbicara pappasang lempu'."},
            "batak_toba": {"kata": "baba", "aksara": "ᯅᯅ", "contoh": "Baba manghatahon hata na uli."},
            "aceh": {"kata": "babah", "contoh": "Babah meututoe saleum rahayu."},
            "banjar": {"kata": "muntung", "contoh": "Muntung bapandir jujur kada badusta."},
            "lampung": {"kata": "banguk", "aksara": "ᰓᰦᰅᰪᰀ", "contoh": "Banguk cawa sai seimbang."},
            "papua": {"kata": "mulut", "contoh": "Mulut ucapkan selamat datang di tanah kami."}
        }
    },
    {
        "indonesia": "gigi", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "untu", "krama_inggil": "waja", "aksara": "ꦮꦗ", "contoh": "Waja pethak resik tansah kaurus."},
            "sunda": {"loma": "huntu", "lemes_batur": "waos", "aksara": "ᮝᮇᮞ", "contoh": "Waos bodas ngagurilap katingalna."},
            "bali": {"andap": "gigi", "alus_singgih": "untun", "aksara": "ᬉꭇᬦꭇᬢꭇᬦ", "contoh": "Untun kagingsir ring upacara potong gigi."},
            "minang": {"kata": "gigi", "contoh": "Gigi putiah baririk rapi."},
            "bugis": {"kata": "isi", "aksara": "ᨕᨗᨔᨗ", "contoh": "Isi mapaccing makkasolang."},
            "makassar": {"kata": "gigi", "contoh": "Gigi kaci' kute' carammeng."},
            "batak_toba": {"kata": "ipon", "aksara": "ᯤᯇᯬᯉ", "contoh": "Ipon na bontar jala tohap."},
            "aceh": {"kata": "gigoe", "contoh": "Gigoe puteh supah manis."},
            "banjar": {"kata": "gigi", "contoh": "Gigi putih mangkilap bagigi timang."},
            "lampung": {"kata": "ipon", "aksara": "ᰀᰤᰎᰩᰍ", "contoh": "Ipon handak berseri."},
            "papua": {"kata": "gigi", "contoh": "Gigi kuat makan sagu dan kelapa tua."}
        }
    },
    {
        "indonesia": "tangan", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "tangan", "krama_inggil": "asta", "aksara": "ꦲꦱ꧀ꦠ", "contoh": "Asta nyembah sujud tanda kurmat."},
            "sunda": {"loma": "leungeun", "lemes_batur": "panangan", "aksara": "ᮕᮔ8ᮔ", "contoh": "Panangan nyepeng payung geulis Tasik."},
            "bali": {"andap": "lima", "alus_singgih": "tangan", "aksara": "ᬢꭇᬗꬓ", "contoh": "Tangan nyakupang ring ajeng dresta."},
            "minang": {"kata": "tangan", "contoh": "Tangan manari piriang balenggok."},
            "bugis": {"kata": "lima", "aksara": "ᨒᨗᨆ", "contoh": "Lima massiddi madduta arung."},
            "makassar": {"kata": "lima", "contoh": "Lima annyakku' pammaling baji'."},
            "batak_toba": {"kata": "tangan", "aksara": "ᯖꯥ꯰ꯪꯉꯉ", "contoh": "Tangan mangalehon ulos tu bere."},
            "aceh": {"kata": "jaroe", "contoh": "Jaroe meutamah bak tari saman."},
            "banjar": {"kata": "tangan", "contoh": "Tangan manganyam tikar purun."},
            "lampung": {"kata": "pungu", "aksara": "ᰎᰪᰅᰪ", "contoh": "Pungu nyulam kain tapis benang mas."},
            "papua": {"kata": "tangan", "contoh": "Tangan terampil mengukir patung Asmat."}
        }
    },
    {
        "indonesia": "kaki", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "sikil", "krama_inggil": "sampeyan", "aksara": "ꦱꦩ꧀ꦥꦺꦪꦤ꧀", "contoh": "Sampeyan lumampah dhumateng pasarean luhur."},
            "sunda": {"loma": "suku", "lemes_batur": "sampean", "aksara": "ᮞᮙ᮪ᮕᮦᮃᮔ", "contoh": "Sampean mapah nyorang jalan satapak."},
            "bali": {"andap": "batis", "alus_singgih": "cokor", "aksara": "ᬘᭀꭇᬓᭀꭇᬭ", "contoh": "Cokor Ida Pedanda ngadeg ring bale gede."},
            "minang": {"kata": "kaki", "contoh": "Kaki maniti pincuran janiah."},
            "bugis": {"kata": "aje'", "aksara": "ᨕᨍᨙ", "contoh": "Aje' jokka ri bola arung."},
            "makassar": {"kata": "jangkang", "contoh": "Jangkang jappa-jappa ri benteng."},
            "batak_toba": {"kata": "pat", "aksara": "ᯇᯖ", "contoh": "Pat mardalan tu dolok timbo."},
            "aceh": {"kata": "gaki", "contoh": "Gaki jak rot bineh krueng."},
            "banjar": {"kata": "batis", "contoh": "Batis bajalan di titian batang ulin."},
            "lampung": {"kata": "caluk", "aksara": "ᰅᰦᰜᰪᰀ", "contoh": "Caluk lapah di renglaya pekon."},
            "papua": {"kata": "kaki", "contoh": "Kaki kokoh daki tebing Lembah Baliem."}
        }
    },
    {
        "indonesia": "hati", "kelas_kata": "nomina", "kategori": "nomina_anatomi",
        "daerah": {
            "jawa": {"ngoko": "ati", "krama_inggil": "manah", "aksara": "ꦩꦤꦃ", "contoh": "Manah ingkang suci tentrem ayem."},
            "sunda": {"loma": "hate", "lemes_batur": "manah", "aksara": "ᮙᮔᮂ", "contoh": "Manah bungah ningali kulawarga rukun."},
            "bali": {"andap": "keneh", "alus_singgih": "kayun", "aksara": "ᬓꭇᬬꭇᬸꭇᬦ", "contoh": "Kayun hening dados sarana sembahyang suci."},
            "minang": {"kata": "hati", "contoh": "Hati sanang dunsanak marantau pulang."},
            "bugis": {"kata": "ati", "aksara": "ᨕᨈᨗ", "contoh": "Ati mapaccing mabbiritta baji'."},
            "makassar": {"kata": "ati", "contoh": "Ati rannu ampa'jari baji'."},
            "batak_toba": {"kata": "roha", "aksara": "ᯒᯬᯂ", "contoh": "Roha na dame mangolu di huta."},
            "aceh": {"kata": "ate", "contoh": "Ate teunang watee seumayang."},
            "banjar": {"kata": "hati", "contoh": "Hati sanang talihat dangsanak akur."},
            "lampung": {"kata": "atei", "aksara": "ᰀᰦᰊᰦᰤ", "contoh": "Atei tenang di bumi ruwa jurai."},
            "papua": {"kata": "hati", "contoh": "Satu hati satu rasa kitorang saudara."}
        }
    },

    # 5. NOMINA ALAM, WAKTU & BENDA
    {
        "indonesia": "air", "kelas_kata": "nomina", "kategori": "nomina_alam",
        "daerah": {
            "jawa": {"ngoko": "banyu", "krama_inggil": "toya", "aksara": "ꦠꦺꦴꦪ", "contoh": "Toya sumber suci ing umbul peninggalan Mataram."},
            "sunda": {"loma": "cai", "lemes_batur": "cai herang", "aksara": "ᮎᮄ", "contoh": "Cai nyusu Curug Cilember tiis tur hérang."},
            "bali": {"andap": "yeh", "alus_singgih": "toya", "aksara": "ᬢꭇᭀꭇᬬ", "contoh": "Toya suci Tirta Empul kasub ring jagat."},
            "minang": {"kata": "aia", "contoh": "Aia janiah sayak landai."},
            "bugis": {"kata": "je'ne'", "aksara": "ᨍᨛᨊᨙ", "contoh": "Je'ne' marica ri pangkep."},
            "makassar": {"kata": "je'ne'", "contoh": "Je'ne' tassi'la' Losari."},
            "batak_toba": {"kata": "aek", "aksara": "ᯀᯀᯩᯄ", "contoh": "Aek Natonang di parik ni dolok."},
            "aceh": {"kata": "ie", "contoh": "Ie krueng aceh ilee bak laot."},
            "banjar": {"kata": "banyu", "contoh": "Banyu riam kanan dingin banar."},
            "lampung": {"kata": "way", "aksara": "ᰓᰦᰤ", "contoh": "Way Sekampung mengalir subur."},
            "papua": {"kata": "air", "contoh": "Air Danau Sentani tenang dan jernih."}
        }
    },
    {
        "indonesia": "api", "kelas_kata": "nomina", "kategori": "nomina_alam",
        "daerah": {
            "jawa": {"ngoko": "geni", "krama_inggil": "latu", "aksara": "ꦭꦠꦸ", "contoh": "Latu abadi Mrapen murub boten naté pejah."},
            "sunda": {"loma": "seuneu", "lemes_batur": "seuneu", "aksara": "ᮞᮩᮔᮩ", "contoh": "Seuneu durukan ngahaneutan hawa peuting."},
            "bali": {"andap": "api", "alus_singgih": "agni", "aksara": "ᬅꭇꬕꭇᬦꭇᬶ", "contoh": "Agni suci Homa sarana panyupatan sarwa ala."},
            "minang": {"kata": "api", "contoh": "Api pambaka kayu manggalagak."},
            "bugis": {"kata": "api", "aksara": "ᨕᨄᨗ", "contoh": "Api marennu ri dapur."},
            "makassar": {"kata": "pepe'", "contoh": "Pepe' makelle' ampalassuk."},
            "batak_toba": {"kata": "api", "aksara": "ᯀᯇᯪ", "contoh": "Api ni tataring pahanas bodari."},
            "aceh": {"kata": "apui", "contoh": "Apui hu meunyala bak teungoh malam."},
            "banjar": {"kata": "api", "contoh": "Api puntung manyala di dandang."},
            "lampung": {"kata": "apui", "aksara": "ᰀᰦᰎᰪᰤ", "contoh": "Apui murub hangatkan beranda."},
            "papua": {"kata": "api", "contoh": "Api unggun menghangatkan malam di honai."}
        }
    },
    {
        "indonesia": "tanah", "kelas_kata": "nomina", "kategori": "nomina_alam",
        "daerah": {
            "jawa": {"ngoko": "lemah", "krama_inggil": "siti", "aksara": "ꦱꦶꦠꦶ", "contoh": "Siti pertiwi Nusantara gemah ripah loh jinawi."},
            "sunda": {"loma": "taneuh", "lemes_batur": "lemah", "aksara": "ᮒᮔᮩᮂ", "contoh": "Taneuh Pasundan subur tatanén merenah."},
            "bali": {"andap": "tanah", "alus_singgih": "pertiwi", "aksara": "ᬧꭇꭇᬢꭇᬶꭇᬯꭇᬶ", "contoh": "Pertiwi agung kalestariang sareng sami."},
            "minang": {"kata": "tanah", "contoh": "Tanah pusako ulayat nagari indak buliah tajua."},
            "bugis": {"kata": "tana", "aksara": "ᨈᨊ", "contoh": "Tana Ogi malempu masagena."},
            "makassar": {"kata": "butta", "contoh": "Butta Kalabbirang Gowa Mangkasara'."},
            "batak_toba": {"kata": "tano", "aksara": "ᯖᯉᯬ", "contoh": "Tano Batak inganan ni parompuan."},
            "aceh": {"kata": "tanoh", "contoh": "Tanoh Rencong meutuah mulia."},
            "banjar": {"kata": "tanah", "contoh": "Tanah banua subur banar batatanaman."},
            "lampung": {"kata": "tanoh", "aksara": "ᰊᰦᰍᰩᰝ", "contoh": "Tanoh lada harum mewangi."},
            "papua": {"kata": "tanah", "contoh": "Tanah Papua surga kecil jatuh ke bumi."}
        }
    },
    {
        "indonesia": "matahari", "kelas_kata": "nomina", "kategori": "nomina_astronomi",
        "daerah": {
            "jawa": {"ngoko": "srengenge", "krama_inggil": "surya", "aksara": "ꦱꦸꦂꦪ", "contoh": "Surya kumenyar madhangi jagat raya."},
            "sunda": {"loma": "panonpoe", "lemes_batur": "panonpoe", "aksara": "ᮕᮔᮧᮔ᮪ᮕᮧᮈ", "contoh": "Panonpoé meleték ti beulah wétan."},
            "bali": {"andap": "matan ai", "alus_singgih": "surya", "aksara": "ᬲꭇꬸꭇꬭꭇꬬ", "contoh": "Surya Raditya nyunarin jagat Bali."},
            "minang": {"kata": "matoari", "contoh": "Matoari tabik manarangangi sawah luluih."},
            "bugis": {"kata": "mata esso", "aksara": "ᨆᨈ ᨕᨛᨔᨚ", "contoh": "Mata esso mabbaja tana Ugi."},
            "makassar": {"kata": "allang", "contoh": "Allang cini' bintoeng mara'."},
            "batak_toba": {"kata": "mata ni ari", "aksara": "ᯔᯖ ᯉᯪ ᯀᯒᯪ", "contoh": "Mata ni ari binsar sian purba."},
            "aceh": {"kata": "mata uroe", "contoh": "Mata uroe teubit cawoeng alam."},
            "banjar": {"kata": "matahari", "contoh": "Matahari mancarong di atas pasar terapung."},
            "lampung": {"kata": "matani rani", "aksara": "ᰕᰦᰊᰦᰍᰤ ᰛᰦᰍᰤ", "contoh": "Matani rani terbit di pucuk gunung."},
            "papua": {"kata": "matahari", "contoh": "Matahari terbit pertama di ufuk timur Merauke."}
        }
    },
    {
        "indonesia": "bulan", "kelas_kata": "nomina", "kategori": "nomina_astronomi",
        "daerah": {
            "jawa": {"ngoko": "rembulan", "krama_inggil": "candra", "aksara": "ꦕꦤ꧀ꦢꦿ", "contoh": "Candra purnama sidhi madhangi ratri."},
            "sunda": {"loma": "bulan", "lemes_batur": "candra", "aksara": "ᮘᮥᮜᮔ", "contoh": "Bulan ngebrak nyaangan palataran peuting."},
            "bali": {"andap": "bulan", "alus_singgih": "sasih", "aksara": "ᬲꭇᬲꭇᬶꭇᬄ", "contoh": "Sasih Kadasa bulan purnama pinih becik."},
            "minang": {"kata": "bulan", "contoh": "Bulan purnamo tarang di ateh gunuang."},
            "bugis": {"kata": "uleng", "aksara": "ᨕᨘᨒᨛ", "contoh": "Uleng leppang mabbiritta masagena."},
            "makassar": {"kata": "bulang", "contoh": "Bulang purnangngang mara' Losari."},
            "batak_toba": {"kata": "bulan", "aksara": "ᯅᯮᯞᯉ", "contoh": "Bulan tula rondang suminar."},
            "aceh": {"kata": "buleuen", "contoh": "Buleuen trang cawoeng saboh gampong."},
            "banjar": {"kata": "bulan", "contoh": "Bulan tarang benderang di banyu sungai."},
            "lampung": {"kata": "bulan", "aksara": "ᰓᰪᰜᰦᰍ", "contoh": "Bulan terang nyinar pekon saibatin."},
            "papua": {"kata": "bulan", "contoh": "Bulan purnama terangi Danau Sentani megah."}
        }
    },
    {
        "indonesia": "laut", "kelas_kata": "nomina", "kategori": "nomina_alam",
        "daerah": {
            "jawa": {"ngoko": "segara", "krama_inggil": "samodra", "aksara": "ꦱꦩꦺꦴꦢꦿ", "contoh": "Samodra kidul ombakipun ageng sanget."},
            "sunda": {"loma": "laut", "lemes_batur": "sagara", "aksara": "ᮞᮌᮛ", "contoh": "Sagara kidul Pelabuhan Ratu endah tur leungit."},
            "bali": {"andap": "pasih", "alus_singgih": "segara", "aksara": "ᬲꭇꬕꭇꬭꭇꬭ", "contoh": "Segara Rupek selat Bali kasub."},
            "minang": {"kata": "lauik", "contoh": "Lauik nan lapeh tapian Padang."},
            "bugis": {"kata": "tasi'", "aksara": "ᨈᨔᨗ", "contoh": "Tasi' Malaka sompe' Pinisi."},
            "makassar": {"kata": "tamparang", "contoh": "Tamparang Kalabbirang Bira."},
            "batak_toba": {"kata": "laut", "aksara": "ᯞᯥᯖ", "contoh": "Laut Sibolga parmianan ni dengke."},
            "aceh": {"kata": "laot", "contoh": "Panglima Laot pimpin nelayan saboh pasi."},
            "banjar": {"kata": "laut", "contoh": "Laut Jawa luas diliwati kapal ba-layar."},
            "lampung": {"kata": "lawok", "aksara": "ᰜᰦᰓᰩᰀ", "contoh": "Lawok Teluk Lampung sugih iwa."},
            "papua": {"kata": "laut", "contoh": "Laut Raja Ampat penuh terumbu karang warna-warni."}
        }
    },
    {
        "indonesia": "gunung", "kelas_kata": "nomina", "kategori": "nomina_geografi",
        "daerah": {
            "jawa": {"ngoko": "gunung", "krama_inggil": "redi", "aksara": "ꦫꦼꦢꦶ", "contoh": "Redi Merapi tansah paring piwulang kauripan."},
            "sunda": {"loma": "gunung", "lemes_batur": "pasir", "aksara": "ᮌᮥᮔᮥᮀ", "contoh": "Gunung Tangkuban Parahu matak waas pamandanganna."},
            "bali": {"andap": "gunung", "alus_singgih": "giri", "aksara": "ᬕꭇꬶꭇꬭꭇꬶ", "contoh": "Giri Agung linggih Ida Bhatara."},
            "minang": {"kata": "gunuang", "contoh": "Gunuang Marapi gagah di Minangkabau."},
            "bugis": {"kata": "bulu'", "aksara": "ᨅᨘᨒᨘ", "contoh": "Bulu' Latimojong timbo masse'."},
            "makassar": {"kata": "moncong", "contoh": "Moncong Lompobattang mallotong."},
            "batak_toba": {"kata": "dolok", "aksara": "ᯑᯬᯞᯬᯄ", "contoh": "Dolok Pusuk Buhit mula ni Batak."},
            "aceh": {"kata": "gunong", "contoh": "Gunong Leuser rimba luah raya."},
            "banjar": {"kata": "gunung", "contoh": "Gunung Meratus rimba asri kalimantan."},
            "lampung": {"kata": "gunung", "aksara": "ᰎᰪᰍᰪᰅ", "contoh": "Gunung Pesagi pucuk tertinggi Lampung."},
            "papua": {"kata": "gunung", "contoh": "Puncak Cartenz Pyramid bersalju abadi."}
        }
    },
    {
        "indonesia": "rumah", "kelas_kata": "nomina", "kategori": "nomina_arsitektur",
        "daerah": {
            "jawa": {"ngoko": "omah", "krama_inggil": "dalem", "aksara": "ꦢꦊꦩ", "contoh": "Kula sowan dhateng dalemipun eyang."},
            "sunda": {"loma": "imah", "lemes_batur": "bumi", "aksara": "ᮘᮥᮙᮤ", "contoh": "Hatur nuhun parantos kersa sumping ka bumi sim kuring."},
            "bali": {"andap": "umah", "alus_singgih": "puri", "aksara": "ᬧꭇꬸꭇꬭꭇꬶ", "contoh": "Titiang ngiring Ida Pedanda mantuk ring gria / puri."},
            "minang": {"kata": "rumah gadang", "contoh": "Rumah Gadang bagonjong sambilan rancak bana."},
            "bugis": {"kata": "bola", "aksara": "ᨅᨚᨒ", "contoh": "Bola arung pitu Saoraja."},
            "makassar": {"kata": "balla'", "contoh": "Balla' Lompoa ri Sungguminasa Gowa."},
            "batak_toba": {"kata": "bagas / ruma bolon", "aksara": "ᯅᯎᯘ", "contoh": "Ruma Bolon gorga Batak na uli."},
            "aceh": {"kata": "rumoh", "contoh": "Rumoh Aceh meupintee pitu."},
            "banjar": {"kata": "rumah", "contoh": "Rumah Bubungan Tinggi lambang martabat Banjar."},
            "lampung": {"kata": "lamban", "aksara": "ᰜᰦᰕᰓᰦᰍ", "contoh": "Lamban sesat balak pepadun."},
            "papua": {"kata": "honai", "contoh": "Honai beratap jerami bundar rapat dan hangat."}
        }
    },
    {
        "indonesia": "pagi", "kelas_kata": "nomina", "kategori": "nomina_waktu",
        "daerah": {
            "jawa": {"ngoko": "esuk", "krama_inggil": "enjang", "aksara": "ꦲꦺꦚ꧀ꦗꦁ", "contoh": "Sugeng enjang para pamiarsa sedaya."},
            "sunda": {"loma": "isuk", "lemes_batur": "enjing", "aksara": "ᮈᮔ᮪ᮏᮤᮀ", "contoh": "Wilujeng enjing wargi Sadaya."},
            "bali": {"andap": "semeng", "alus_singgih": "semeng", "aksara": "ᬲꭇꬾꭇꬾꭇꬲ", "contoh": "Rahajeng semeng semeton sami ring Bali."},
            "minang": {"kata": "pagi", "contoh": "Salamaik pagi dunsanak di ranah Minang."},
            "bugis": {"kata": "ele'", "aksara": "ᨕᨙᨒᨙ", "contoh": "Salama' ele' to malebbi'e."},
            "makassar": {"kata": "bari'basa", "contoh": "Salama' bari'basa sari'battang."},
            "batak_toba": {"kata": "manogot", "aksara": "ᯔꯥᯉ꯬ᯎᯬᯖ", "contoh": "Horas manogot di hita saluhutna."},
            "aceh": {"kata": "beungoh", "contoh": "Saleum beungoh keu biek meutuah."},
            "banjar": {"kata": "baisukan", "contoh": "Salawat baisukan gasan urang banua."},
            "lampung": {"kata": "mepagi", "aksara": "ᰕᰦᰎᰦᰎᰤ", "contoh": "Selamat mepagi pun kelamo."},
            "papua": {"kata": "pagi", "contoh": "Selamat pagi kawan samua di tanah Papua."}
        }
    },
    {
        "indonesia": "malam", "kelas_kata": "nomina", "kategori": "nomina_waktu",
        "daerah": {
            "jawa": {"ngoko": "wengi", "krama_inggil": "dalu", "aksara": "ꦢꦭꦸ", "contoh": "Sugeng dalu para rawuh sedaya."},
            "sunda": {"loma": "peuting", "lemes_batur": "wengi", "aksara": "ᮝᮨ8ᮤ", "contoh": "Wilujeng wengi kasadayana warga Parahyangan."},
            "bali": {"andap": "peteng", "alus_singgih": "wengi", "aksara": "ᬯꭇꬾꭇꬶ", "contoh": "Rahajeng wengi semeton sami."},
            "minang": {"kata": "malam", "contoh": "Salamaik malam dunsanak di rantau."},
            "bugis": {"kata": "benni", "aksara": "ᨅᨛᨊᨗ", "contoh": "Salama' benni ri tana Ugi."},
            "makassar": {"kata": "banggia", "contoh": "Salama' banggia sari'battang."},
            "batak_toba": {"kata": "borngin", "aksara": "ᯅᯬᯒ᯲꯰ᯪᯉ", "contoh": "Horas borngin di hita saluhutna."},
            "aceh": {"kata": "malam", "contoh": "Saleum malam keu biek meutuah."},
            "banjar": {"kata": "malam", "contoh": "Salawat malam gasan urang Banua."},
            "lampung": {"kata": "debing", "aksara": "ᰌᰦᰓᰤᰅ", "contoh": "Selamat debing gham seunyini."},
            "papua": {"kata": "malam", "contoh": "Selamat malam kitorang samua."}
        }
    },

    # 6. NOMINA KELUARGA & KEKERABATAN
    {
        "indonesia": "ayah", "kelas_kata": "nomina", "kategori": "nomina_keluarga",
        "daerah": {
            "jawa": {"ngoko": "bapak", "krama_inggil": "rama", "aksara": "ꦫꦩ", "contoh": "Rama paring wejangan bab unggah-ungguh."},
            "sunda": {"loma": "bapa", "lemes_batur": "rama", "aksara": "ᮛᮙ", "contoh": "Pun rama mulih ti kantor sonten."},
            "bali": {"andap": "bapa", "alus_singgih": "aji", "aksara": "ᬅꭇᬚꭇᬶ", "contoh": "Ida Aji Pedanda wicaksana ngabih krama."},
            "minang": {"kata": "apak", "contoh": "Apak batani di sawah gaduang."},
            "bugis": {"kata": "ambo'", "aksara": "ᨕᨅᨚ", "contoh": "Ambo'ku arung pitu."},
            "makassar": {"kata": "bapa' / mangge", "contoh": "Mangge toa passuroang lempu'."},
            "batak_toba": {"kata": "damang / ama", "aksara": "ᯀᯔ", "contoh": "Damang parpoda na denggan tu hami."},
            "aceh": {"kata": "ayah / abu", "contoh": "Ayah geupakat jak u blang."},
            "banjar": {"kata": "abah", "contoh": "Abah mangayuh jukung ka huma."},
            "lampung": {"kata": "apak / bapak", "aksara": "ᰀᰦᰎᰦᰀ", "contoh": "Apak nuntun jurai anak buah."},
            "papua": {"kata": "bapa", "contoh": "Bapa ajar berburu dan buat perahu dayung."}
        }
    },
    {
        "indonesia": "ibu", "kelas_kata": "nomina", "kategori": "nomina_keluarga",
        "daerah": {
            "jawa": {"ngoko": "mbok", "krama_inggil": "ibu", "aksara": "ꦲꦶꦧꦸ", "contoh": "Ibu nyerat bathik tulis motif parang."},
            "sunda": {"loma": "indung", "lemes_batur": "pun biang", "aksara": "ᮕᮥᮔ᮪ ᮘᮤᮃᮀ", "contoh": "Pun biang nuju ngadamel kueh ali agrem amis."},
            "bali": {"andap": "meme", "alus_singgih": "biang", "aksara": "ᬩꭇᬶꭇᬅꭇᬗ", "contoh": "Ida Biang ngaturang canang sari suci."},
            "minang": {"kata": "mande / amak", "contoh": "Mande manyiapkan nasi bajamba."},
            "bugis": {"kata": "indo'", "aksara": "ᨕᨗᨉᨚ", "contoh": "Indo'ku massulap lipa' sabbe."},
            "makassar": {"kata": "amma'", "contoh": "Amma' ampa'risi ana' cucu."},
            "batak_toba": {"kata": "dainang / ina", "aksara": "ᯤꯉ", "contoh": "Dainang mangalehon ulos holong."},
            "aceh": {"kata": "mak / poma", "contoh": "Poma meutamah bak kenduri blang."},
            "banjar": {"kata": "uma / kuitan", "contoh": "Uma maulah wadai sarimuka manis."},
            "lampung": {"kata": "emak / inak", "aksara": "ᰀᰦᰕᰦᰀ", "contoh": "Emak nyulam tapis peradaban."},
            "papua": {"kata": "mama", "contoh": "Mama ajar cucu cara anyam noken kulit kayu."}
        }
    },
    {
        "indonesia": "anak", "kelas_kata": "nomina", "kategori": "nomina_keluarga",
        "daerah": {
            "jawa": {"ngoko": "anak", "krama_inggil": "putra", "aksara": "ꦥꦸꦠꦿ", "contoh": "Putra panjenengan sregep sinau basa Jawa."},
            "sunda": {"loma": "budak", "lemes_batur": "putra", "aksara": "ᮕᮥᮒ᮪ᮛ", "contoh": "Putra pun bapa nuju diajar di pasantren."},
            "bali": {"andap": "pianak", "alus_singgih": "oka", "aksara": "ᬑꭇꬓ", "contoh": "Ida Oka sampun tamat masekolah."},
            "minang": {"kata": "anak", "contoh": "Anak pisang mananti di alek gadang."},
            "bugis": {"kata": "ana'", "aksara": "ᨕᨊ", "contoh": "Ana' matoa magguru adeq."},
            "makassar": {"kata": "ana'", "contoh": "Ana' cucu sipakalebbi' ri baruga."},
            "batak_toba": {"kata": "anak / boru", "aksara": "ᯀᯉᯄ", "contoh": "Anakkon hi do hamoraon di au."},
            "aceh": {"kata": "aneuk", "contoh": "Aneuk nanggroe pahlawan bangsa."},
            "banjar": {"kata": "kakanak", "contoh": "Kakanak rami bakunyung di banyu sungai."},
            "lampung": {"kata": "sanak", "aksara": "ᰞᰦᰍᰦᰀ", "contoh": "Sanak bujang gadis belajar adab."},
            "papua": {"kata": "anak", "contoh": "Anak-anak riang bermain di pantai Raja Ampat."}
        }
    },
    {
        "indonesia": "kakek", "kelas_kata": "nomina", "kategori": "nomina_keluarga",
        "daerah": {
            "jawa": {"ngoko": "mbah kakung", "krama_inggil": "eyang kakung", "aksara": "ꦲꦺꦪꦁ ꦏꦏꦸꦁ", "contoh": "Eyang kakung paring cariyos sejarah kraton."},
            "sunda": {"loma": "aki", "lemes_batur": "eyang pameget", "aksara": "ᮃᮊᮤ", "contoh": "Aki nuju calik di teras bari ngopi."},
            "bali": {"andap": "pepek", "alus_singgih": "kaki / niang", "aksara": "ᬓꭇꬓꭇꬶ", "contoh": "Kaki titiang uning ring satua Bali buhun."},
            "minang": {"kata": "inyiak / angku", "contoh": "Angku Datuak bijaksano manuntun nagari."},
            "bugis": {"kata": "lato'", "aksara": "ᨒᨈᨚ", "contoh": "Lato'ku arung pitu."},
            "makassar": {"kata": "toa", "contoh": "Toa baji' parumpuanna."},
            "batak_toba": {"kata": "ompu doli", "aksara": "ᯀᯬᯔ᯲ᯇᯮ ᯑᯬᯞᯪ", "contoh": "Ompu doli parpoda na denggan tu hita."},
            "aceh": {"kata": "nek / yahnek", "contoh": "Yahnek peugah haba raja masa jameun."},
            "banjar": {"kata": "kai", "contoh": "Kai katuju mangayuh jukung ka huma."},
            "lampung": {"kata": "umpu", "aksara": "ᰀᰪᰕᰎᰪ", "contoh": "Umpu nuntun anak cucu lamban agung."},
            "papua": {"kata": "tete", "contoh": "Tete cerita sejarah suku di rumah adat."}
        }
    },
    {
        "indonesia": "nenek", "kelas_kata": "nomina", "kategori": "nomina_keluarga",
        "daerah": {
            "jawa": {"ngoko": "mbah putri", "krama_inggil": "eyang putri", "aksara": "ꦲꦺꦪꦁ ꦥꦸꦠꦿꦶ", "contoh": "Eyang putri mundhut bathik tulis Yogya."},
            "sunda": {"loma": "nini", "lemes_batur": "eyang istri", "aksara": "ᮔᮤᮔᮤ", "contoh": "Nini ngadamel kueh ali agrem amis."},
            "bali": {"andap": "dadong", "alus_singgih": "niang", "aksara": "ᬤꭇꭀꭇᬗ", "contoh": "Dadong ngae jaja uli manis."},
            "minang": {"kata": "anduang / niniak", "contoh": "Anduang mambuek randang baluik."},
            "bugis": {"kata": "nene'", "aksara": "ᨊᨛᨊᨙ", "contoh": "Nene'ku matoa mabbicara adeq."},
            "makassar": {"kata": "nene'", "contoh": "Nene' ampa'risi ana' cucu."},
            "batak_toba": {"kata": "ompu boru", "aksara": "ᯀᯬᯔ᯲ᯇᯮ ᯅᯬᯒᯮ", "contoh": "Ompu boru mangalehon ulos holong."},
            "aceh": {"kata": "maknek", "contoh": "Maknek meusunteng ija sawak."},
            "banjar": {"kata": "nini", "contoh": "Nini maulah wadai pais pisang manis."},
            "lampung": {"kata": "bai", "aksara": "ᰓᰦᰤ", "contoh": "Bai nyulam tapis Lampung telaten."},
            "papua": {"kata": "nene", "contoh": "Nene ajar cucu cara anyam noken."}
        }
    },
    {
        "indonesia": "sahabat", "kelas_kata": "nomina", "kategori": "nomina_sosial",
        "daerah": {
            "jawa": {"ngoko": "kanca", "krama_inggil": "mitra", "aksara": "ꦩꦶꦠꦿ", "contoh": "Mitra pitepangan saking kitha Surakarta."},
            "sunda": {"loma": "sobat", "lemes_batur": "mitra", "aksara": "ᮙᮤᮒ᮪ᮛ", "contoh": "Sobat dalit silih tulungan dina kasusah."},
            "bali": {"andap": "timpal", "alus_singgih": "sawitra", "aksara": "ᬲꭇ꬯ꭇꬶꭇꬢꭇꬭ", "contoh": "Sawitra becik satya ring semaya urip."},
            "minang": {"kata": "dunsanak / kawan", "contoh": "Dunsanak sabana karib di rantau urang."},
            "bugis": {"kata": "bela", "aksara": "ᨅᨙᨒ", "contoh": "Bela masse' sipakaraja."},
            "makassar": {"kata": "agang", "contoh": "Agang baji' sipakalebbi'."},
            "batak_toba": {"kata": "ale-ale", "aksara": "ᯀᯞᯩᯀᯞᯩ", "contoh": "Ale-ale na satia ro di ujungna."},
            "aceh": {"kata": "rakan", "contoh": "Rakan meutaloe beutrok masa tuha."},
            "banjar": {"kata": "kawal", "contoh": "Kawal akrab mulai kakanak sa-kampung."},
            "lampung": {"kata": "kanca", "aksara": "ᰀᰦᰍᰅᰦ", "contoh": "Kanca buhimpun sesat balak."},
            "papua": {"kata": "kawan", "contoh": "Kawan setia sehati satu jiwa di perantauan."}
        }
    },

    # 7. ADJEKTIVA SIFAT & KUALITAS
    {
        "indonesia": "besar", "kelas_kata": "adjektiva", "kategori": "adjektiva_ukuran",
        "daerah": {
            "jawa": {"ngoko": "gedhe", "krama_inggil": "ageng", "aksara": "ꦲꦒꦼꦁ", "contoh": "Gadhah tekad ageng ngayahi jejibahan luhur."},
            "sunda": {"loma": "gede", "lemes_batur": "ageung", "aksara": "ᮃᮌᮩᮀ", "contoh": "Gunung Gede ageung tur endah katingalna."},
            "bali": {"andap": "gede", "alus_singgih": "ageng", "aksara": "ᬅꭇꬕꭇꬾꭇꬲ", "contoh": "Pura Besakih pura ageng pinih utama ring Bali."},
            "minang": {"kata": "gadang", "contoh": "Baralek gadang ditandoi tabuah larangan."},
            "bugis": {"kata": "lompo", "aksara": "ᨒᨚᨄᨚ", "contoh": "Bola lompo Saoraja arung pitu."},
            "makassar": {"kata": "lompo", "contoh": "Balla' Lompoa ri Sungguminasa Gowa."},
            "batak_toba": {"kata": "bolon", "aksara": "ᯅᯬᯞᯬᯉ", "contoh": "Ruma Bolon inganan ni Raja Sisingamangaraja."},
            "aceh": {"kata": "raya", "contoh": "Meuseujid Raya Baiturrahman megah meukilau."},
            "banjar": {"kata": "ganal", "contoh": "Pohon ulin ganal tumbuh di pedalaman Meratus."},
            "lampung": {"kata": "balak", "aksara": "ᰓᰦᰜᰦᰀ", "contoh": "Sesat balak tempat rembuk pepadun."},
            "papua": {"kata": "besar", "contoh": "Pohon matoa besar berbuah manis di hutan."}
        }
    },
    {
        "indonesia": "kecil", "kelas_kata": "adjektiva", "kategori": "adjektiva_ukuran",
        "daerah": {
            "jawa": {"ngoko": "cilik", "krama_inggil": "alit", "aksara": "ꦲꦭꦶꦠ", "contoh": "Lare alit sregep sinau lan bekti dhateng tiyang sepuh."},
            "sunda": {"loma": "leutik", "lemes_batur": "alit", "aksara": "ᮃᮜᮤᮒ", "contoh": "Bumi alit nanging haneuteun tur pinuh kabagjaan."},
            "bali": {"andap": "cenik", "alus_singgih": "alit", "aksara": "ᬅꭇꬮꭇꬶꭇꬢ", "contoh": "Anak alit seneng melajah ngigel ring banjar."},
            "minang": {"kata": "ketek", "contoh": "Ketek taraniayo gadang tabangkuang."},
            "bugis": {"kata": "baiccu'", "aksara": "ᨅᨕᨗᨌᨘ", "contoh": "Lopi baiccu' masiga ri tasi'."},
            "makassar": {"kata": "ca'di", "contoh": "Juku' ca'di ampalassuk."},
            "batak_toba": {"kata": "metmet", "aksara": "ᯔᯩᯖ᯲ᯔᯩᯖ", "contoh": "Dakdanak metmet marsiajar tortor."},
            "aceh": {"kata": "cut", "contoh": "Cut Nyak Dhien pahlawan srikandi Aceh."},
            "banjar": {"kata": "halus", "contoh": "Kakanak halus katuju bamainan di lanting."},
            "lampung": {"kata": "luting", "aksara": "ᰜᰪᰊᰤᰅ", "contoh": "Anak luting pandai mengaji."},
            "papua": {"kata": "kecil", "contoh": "Perahu kecil melaju lincah di selat karang."}
        }
    },
    {
        "indonesia": "baik", "kelas_kata": "adjektiva", "kategori": "adjektiva_karakter",
        "daerah": {
            "jawa": {"ngoko": "apik", "krama_inggil": "sae", "aksara": "ꦱꦲꦺ", "contoh": "Tumindak sae badhe ngundhuh wohing kabecikan."},
            "sunda": {"loma": "hade", "lemes_batur": "sae", "aksara": "ᮞᮃᮦ", "contoh": "Lampah sae bakal mawa kaberkahan dina kahirupan."},
            "bali": {"andap": "luwung", "alus_singgih": "becik", "aksara": "ᬩꭇꬾꭇ꬘ꭇꬶꭇꬡ", "contoh": "Karma becik ngawinang moksa lan katentreman jagat."},
            "minang": {"kata": "elok", "contoh": "Elok baso katuju urang, elok parangai sanang basamo."},
            "bugis": {"kata": "madeceng", "aksara": "ᨆᨉᨙᨌᨛ", "contoh": "Pangadereng madeceng pattentu atuwong."},
            "makassar": {"kata": "baji'", "contoh": "Tau baji' ma'pappasang lempu'."},
            "batak_toba": {"kata": "denggan", "aksara": "ᯑᯩ꯰ᯎꯉ", "contoh": "Denggan ma parsaoran ni dongan sabutuha."},
            "aceh": {"kata": "jroh", "contoh": "Hate jroh hudep teunang dunya aheurat."},
            "banjar": {"kata": "ba-ik", "contoh": "Karakter ba-ik marikit kasayangan warga."},
            "lampung": {"kata": "helau", "aksara": "ᰝᰦᰜᰦᰪ", "contoh": "Piil pesenggiri helau budi pekerti."},
            "papua": {"kata": "baik", "contoh": "Hati baik kitorang hidup damai bergandengan tangan."}
        }
    },
    {
        "indonesia": "lezat", "kelas_kata": "adjektiva", "kategori": "adjektiva_rasa",
        "daerah": {
            "jawa": {"ngoko": "enak", "krama_inggil": "eco", "aksara": "ꦲꦺꦕꦺꦴ", "contoh": "Dhahar sekul liwet gurih raosipun eco sanget."},
            "sunda": {"loma": "ngeunah", "lemes_batur": "raos", "aksara": "ᮛᮇᮞ", "contoh": "Tuang sareng sambel tarasi raos pisan."},
            "bali": {"andap": "jaen", "alus_singgih": "becik", "aksara": "ᬚꭇꬅꭇꬾꭇꬦ", "contoh": "Ayam betutu Gilimanuk puniki jaen pesan raosnyane."},
            "minang": {"kata": "lamak", "contoh": "Randang Daging Payakumbuh lamak bana taraso di lidah."},
            "bugis": {"kata": "nyameng", "aksara": "ᨍᨆᨛ", "contoh": "Nasu palekko Bebek nyameng ladde'."},
            "makassar": {"kata": "nyama", "contoh": "Coto Makassar nyama sikatutui."},
            "batak_toba": {"kata": "tabo", "aksara": "ᯖᯅᯬ", "contoh": "Tabo do dengke naniarsik di haroan."},
            "aceh": {"kata": "mangat", "contoh": "Mie Aceh kepiting mangat that rasa kuah."},
            "banjar": {"kata": "nyaman", "contoh": "Soto Banjar kuah susu nyaman banar dicucul perkedel."},
            "lampung": {"kata": "bangik", "aksara": "ᰓᰦᰅᰤᰀ", "contoh": "Seruit tempoyak ikan baung bangik temon."},
            "papua": {"kata": "enak", "contoh": "Papeda ikan kuah kuning enak sekali dimakan panas."}
        }
    },
    {
        "indonesia": "manis", "kelas_kata": "adjektiva", "kategori": "adjektiva_rasa",
        "daerah": {
            "jawa": {"ngoko": "legi", "krama_inggil": "legi", "aksara": "ꦭꦼꦒꦶ", "contoh": "Wedang ronde anget legi sumrambah."},
            "sunda": {"loma": "amis", "lemes_batur": "amis", "aksara": "ᮃᮙᮤᮞ", "contoh": "Dodol Garut amis pulen dipikaresep sarerea."},
            "bali": {"andap": "manis", "alus_singgih": "manis", "aksara": "ᬫꭇꬓꭇꬶꭇꬡ", "contoh": "Jaja laklak manis magula bali."},
            "minang": {"kata": "manih", "contoh": "Kalamai manis legit khas Minangkabau."},
            "bugis": {"kata": "cenning", "aksara": "ᨌᨛᨊᨗ", "contoh": "Kue baje' cenning marica."},
            "makassar": {"kata": "te'ne", "contoh": "Barongko te'ne malepang."},
            "batak_toba": {"kata": "tonggi", "aksara": "ᯖᯬ꯰ꯎᯪ", "contoh": "Tonggi ni gula siaek pusu-pusu."},
            "aceh": {"kata": "mangat", "contoh": "Meuseukat mangat manis lambang adat mulia."},
            "banjar": {"kata": "manis", "contoh": "Wadai bingka kentang manis harum pandan."},
            "lampung": {"kata": "manis", "aksara": "ᰕᰦᰍᰤᰠ", "contoh": "Kopi Lampung manis gulo enau."},
            "papua": {"kata": "manis", "contoh": "Kelapa muda manis segar diteguk siang hari."}
        }
    },
    {
        "indonesia": "cepat", "kelas_kata": "adjektiva", "kategori": "adjektiva_kecepatan",
        "daerah": {
            "jawa": {"ngoko": "cepet", "krama_inggil": "enggal", "aksara": "ꦲꦺꦁꦒꦭ꧀", "contoh": "Lumampah enggal supados boten kasep."},
            "sunda": {"loma": "gancang", "lemes_batur": "enggal", "aksara": "ᮌᮔ᮪ᮎᮀ", "contoh": "Hayu urang gancang mios bilih kabujeng hujan."},
            "bali": {"andap": "enggal", "alus_singgih": "tancep", "aksara": "ᬏꭇꬾꭇꬕꭇꬭ", "contoh": "Margiang pakaryan puniki mangda tancep puput."},
            "minang": {"kata": "capek", "contoh": "Capek kaki ringan tangan bagotong royong."},
            "bugis": {"kata": "madoja / masiga", "aksara": "ᨆᨔᨗᨁ", "contoh": "Masiga sompe' lopi Pinisi ri tasi'."},
            "makassar": {"kata": "lante' / kence'", "contoh": "Kence' a'lari ampa'jari juara."},
            "batak_toba": {"kata": "hatop", "aksara": "ᯂᯖᯬᯇ", "contoh": "Hatop ma ho ro tuson dongan."},
            "aceh": {"kata": "bagah", "contoh": "Bagah-bagah jak u pasie beu trok."},
            "banjar": {"kata": "lakas", "contoh": "Lakas kayuh jukungnya takutan kaburu pasang."},
            "lampung": {"kata": "geluk", "aksara": "ᰎᰦᰜᰪᰀ", "contoh": "Geluk mulang debingi dapok hujan."},
            "papua": {"kata": "cepat", "contoh": "Dayung perahu cepat melaju di atas ombak."}
        }
    },
    {
        "indonesia": "pintar", "kelas_kata": "adjektiva", "kategori": "adjektiva_kecerdasan",
        "daerah": {
            "jawa": {"ngoko": "pinter", "krama_inggil": "wicaksana", "aksara": "ꦥꦶꦤ꧀ꦠꦼꦂ", "contoh": "Siswa pinter sinau unggah-ungguh lan tata krama."},
            "sunda": {"loma": "pinter", "lemes_batur": "binangkit", "aksara": "ᮕᮤᮔ᮪ᮒᮨᮁ", "contoh": "Murid pinter tur binangkit dina sagala widang."},
            "bali": {"andap": "dueg", "alus_singgih": "pradnya", "aksara": "ᬤꭇꬸꭇꬅꭇꬕ", "contoh": "Siswa pradnya waged nyurat aksara Bali ring lontar."},
            "minang": {"kata": "cadiak", "contoh": "Cadiak pandai tampek batanyo urang nagari."},
            "bugis": {"kata": "macca", "aksara": "ᨆᨌ", "contoh": "Macca na malempu syarat pitu arung."},
            "makassar": {"kata": "carakde", "contoh": "Tau carakde baji' parumpuanna."},
            "batak_toba": {"kata": "marbisuk", "aksara": "ᯔꯥᯒ᯲ᯅꯪᯪᯘꯪᯮᯄ", "contoh": "Raja na marbisuk mamutus parbadaan."},
            "aceh": {"kata": "carong", "contoh": "Ureueng carong pubuet nanggroe meuhase."},
            "banjar": {"kata": "harat / pintar", "contoh": "Harat banar inya mangaji kitab kuning."},
            "lampung": {"kata": "pandai", "aksara": "ᰎᰦᰍᰌᰦᰤ", "contoh": "Bujang pandai sastra Lampung klasik."},
            "papua": {"kata": "jago / pintar", "contoh": "Anak Papua pintar ukir dan hafal cerita leluhur."}
        }
    },
    {
        "indonesia": "indah", "kelas_kata": "adjektiva", "kategori": "adjektiva_estetika",
        "daerah": {
            "jawa": {"ngoko": "apik", "krama_inggil": "endah", "aksara": "ꦲꦺꦤ꧀ꦢꦃ", "contoh": "Sesawangan Gunung Bromo endah sanget."},
            "sunda": {"loma": "alus", "lemes_batur": "asri", "aksara": "ᮃᮞ᮪ᮛᮤ", "contoh": "Pamandangan alam Parahyangan endah tur asri."},
            "bali": {"andap": "luwung", "alus_singgih": "asri", "aksara": "ᬮꭇꬸꭇ꬯ꭇꬸꭇꬲ", "contoh": "Pantai Kuta asri pisan ritatkala surya tenggelam."},
            "minang": {"kata": "rancak", "contoh": "Rancak bana pamandangan Ngarai Sianok Bukittinggi."},
            "bugis": {"kata": "magello", "aksara": "ᨆᨁᨛᨒᨚ", "contoh": "Magello lita'na tana Bone riolo."},
            "makassar": {"kata": "baji' rupanna", "contoh": "Baji' rupanna Pantai Losari wattu banggia."},
            "batak_toba": {"kata": "uli", "aksara": "ᯥᯞꯪᯪ", "contoh": "Uli ni rupa ni Tao Toba sian Dolok Martimbang."},
            "aceh": {"kata": "ceudah", "contoh": "Ceudah that pemandangan panton reuleueh."},
            "banjar": {"kata": "bungas", "contoh": "Bungas langkar kain sasirangan babungaan."},
            "lampung": {"kata": "helau", "aksara": "ᰝᰦᰜᰦᰪ", "contoh": "Helau pemandangan Way Kambas."},
            "papua": {"kata": "indah", "contoh": "Raja Ampat indah bagai surga di bumi Papua."}
        }
    },

    # 8. UNGKAPAN, SAPAAN & KESANTUNAN
    {
        "indonesia": "terima kasih", "kelas_kata": "ungkapan", "kategori": "ungkapan_kesantunan",
        "daerah": {
            "jawa": {"ngoko": "kesuwun", "krama_inggil": "matur nuwun", "aksara": "ꦩꦠꦸꦂ ꦤꦸꦮꦸꦤ꧀", "contoh": "Matur nuwun sanget awit sedaya pambiyantu panjenengan."},
            "sunda": {"loma": "nuhun", "lemes_batur": "hatur nuhun", "aksara": "ᮠᮒᮥᮁ ᮔᮥᮠᮥᮔ", "contoh": "Hatur nuhun pisan parantos kersa sumping ka rorompok."},
            "bali": {"andap": "suksma", "alus_singgih": "matur suksma", "aksara": "ᬫꭇꬢꭇꬸꭇꬭ ᬲꭇꬸꭇꬓꭇꬡꭇꬾꭇꬓ", "contoh": "Matur suksma banget ring kasukserahan semeton sami."},
            "minang": {"kata": "tarimo kasih", "contoh": "Tarimo kasih banyak ateh sagalo bantuan dunsanak."},
            "bugis": {"kata": "kurru sumange", "aksara": "ᨀᨘᨑᨘ ᨔᨘᨆᨂᨙ", "contoh": "Kurru sumange ri atuwong madecengta."},
            "makassar": {"kata": "tarima kase'", "contoh": "Tarima kase' sari'battang samua."},
            "batak_toba": {"kata": "mauliate", "aksara": "ᯔᯥᯞꯪᯪꯀꯥᯖᯩ", "contoh": "Mauliate godang di nasa pambahenanmuna."},
            "aceh": {"kata": "teurimong gaseh", "contoh": "Teurimong gaseh biek meutuah bandum."},
            "banjar": {"kata": "tarima kasih", "contoh": "Tarima kasih banyak gasan kakawalan samunyaan."},
            "lampung": {"kata": "nalom nihan", "aksara": "ᰍᰦᰜᰩᰕ ᰍᰤᰝᰦᰍ", "contoh": "Nalom nihan atas bantuan pepadun."},
            "papua": {"kata": "terima kasih banyak", "contoh": "Terima kasih banyak kawan atas kebaikan hati."}
        }
    },
    {
        "indonesia": "maaf", "kelas_kata": "ungkapan", "kategori": "ungkapan_kesantunan",
        "daerah": {
            "jawa": {"ngoko": "ngapura", "krama_inggil": "nyuwun pangapunten", "aksara": "ꦚꦸꦮꦸꦤ꧀ ꦥꦔꦥꦸꦤ꧀ꦠꦼꦤ꧀", "contoh": "Nyuwun pangapunten menawi wonten kalepatan tutur basa."},
            "sunda": {"loma": "hapunten", "lemes_batur": "punten", "aksara": "ᮠᮕᮥᮔ᮪ᮒᮨᮔ", "contoh": "Hapunten anu kasuhun bilih aya kalepatan laku lampah."},
            "bali": {"andap": "ngampurayang", "alus_singgih": "nunas ampura", "aksara": "ᬦꭇꬸꭇꬦꭇꬰꭇꬡ ᬅꭇꬾꭇ꬧ꭇꬸꭇꬭ", "contoh": "Titiang nunas ampura pisan ring geng rena sinampura."},
            "minang": {"kata": "minta maaf", "contoh": "Minta maaf ambo kok ado salah kato jo parangai."},
            "bugis": {"kata": "tabe'", "aksara": "ᨈᨅᨙ", "contoh": "Tabe' puang, taddampengekengngi kalepakengku."},
            "makassar": {"kata": "tabe'", "contoh": "Tabe' sari'battang, ammotere'ma' riolo."},
            "batak_toba": {"kata": "santabi", "aksara": "ᯘꯥꯉ᯲ᯖꯥᯅꯪᯪ", "contoh": "Santabi di angka raja dohot sude hula-hula."},
            "aceh": {"kata": "tulong meuafe", "contoh": "Tulong meuafe beurayeuk ate bak salah lon."},
            "banjar": {"kata": "minta rela", "contoh": "Minta rela minta halal lahir batin dunia akhirat."},
            "lampung": {"kata": "tabik pun", "aksara": "ᰊᰦᰓᰤᰀ ᰎᰪᰍ", "contoh": "Tabik pun nalom ngelimpahkan pangampun."},
            "papua": {"kata": "minta maaf", "contoh": "Minta maaf bila ada salah bicara dengan sodara."}
        }
    },
    {
        "indonesia": "apa kabar", "kelas_kata": "ungkapan", "kategori": "ungkapan_sapaan",
        "daerah": {
            "jawa": {"ngoko": "piye kabare", "krama_inggil": "kados pundi pawartosipun", "aksara": "ꦏꦢꦺꦴꦱ꧀ ꦥꦸꦤ꧀ꦢꦶ ꦥꦮꦂꦠꦺꦴꦱꦶꦥꦸꦤ꧀", "contoh": "Kados pundi pawartosipun panjenengan sakulawarga?"},
            "sunda": {"loma": "kumaha damang", "lemes_batur": "kumaha damang", "aksara": "ᮊᮥᮙᮠ ᮓᮙᮀ", "contoh": "Kumaha damang sadayana wargi Bandung?"},
            "bali": {"andap": "kenken kabare", "alus_singgih": "punapi gatrane", "aksara": "ᬧꭇꬸꭇꬦꭇꬰꭇꬶ ᬕꭇꬢꭇꬭꭇꬓᬾ", "contoh": "Punapi gatrane semeton sami, mangda sami rahayu."},
            "minang": {"kata": "baa kaba", "contoh": "Baa kaba sanak di rantau, lai sihaik sanang?"},
            "bugis": {"kata": "aga kareba", "aksara": "ᨕᨁ ᨀᨑᨙᨅ", "contoh": "Aga kareba silessurengku maraja?"},
            "makassar": {"kata": "apa kareba", "contoh": "Apa kareba sari'battang ri Gowa?"},
            "batak_toba": {"kata": "beha barita", "aksara": "ᯅᯩᯂ ᯅꯥꯒꯪᯪᯖ", "contoh": "Beha barita hamu sude dongan tubu?"},
            "aceh": {"kata": "pane haba", "contoh": "Pane haba gata nyang baro troh?"},
            "banjar": {"kata": "kayapa habar", "contoh": "Kayapa habar buhan pian di banua sabarataan?"},
            "lampung": {"kata": "api kabar", "aksara": "ᰀᰦᰎᰤ ᰀᰦᰓᰦᰛ", "contoh": "Api kabar bujang gadis seunyini?"},
            "papua": {"kata": "bagaimana kabar", "contoh": "Bagaimana kabar kitorang samua, sehat-sehat to?"}
        }
    },
    {
        "indonesia": "selamat jalan", "kelas_kata": "ungkapan", "kategori": "ungkapan_sapaan",
        "daerah": {
            "jawa": {"ngoko": "sugeng tindak", "krama_inggil": "sugeng tindak", "aksara": "ꦱꦸꦒꦼꦁ ꦠꦶꦤ꧀ꦢꦏ꧀", "contoh": "Sugeng tindak mugi tansah pinayungan karahayon."},
            "sunda": {"loma": "wilujeng angkat", "lemes_batur": "wilujeng angkat", "aksara": "ᮝᮤᮜᮥᮏᮨᮀ ᮃᮀᮊᮒ", "contoh": "Wilujeng angkat mugia salamet dugi ka tujuan."},
            "bali": {"andap": "rahajeng memargi", "alus_singgih": "rahajeng memargi", "aksara": "ᬭꭇꬰꭇ꬚ꭇꬾꭇꬲ ᬫꭇꬾꭇꬭꭇꬕꭇꬶ", "contoh": "Rahajeng memargi semeton, mangda nemu karahayuan."},
            "minang": {"kata": "salamaik jalan", "contoh": "Salamaik jalan dunsanak, salamaik sampai di rantau."},
            "bugis": {"kata": "salama' lao", "aksara": "ᨔᨒᨆ ᨒᨕᨚ", "contoh": "Salama' lao ri pammanakkeng."},
            "makassar": {"kata": "salama' a'lampang", "contoh": "Salama' a'lampang sari'battang."},
            "batak_toba": {"kata": "horas di pardalanan", "aksara": "ᯂᯬᯒꯥᯘ ᯑꯪᯪ ᯇꯥꯒꯪᯪꯒꯥꯞꯥꯉ", "contoh": "Horas ma di pardalananmuna tu parserahan."},
            "aceh": {"kata": "saleum meugisa", "contoh": "Saleum meugisa beu seulamat trok bak gampong."},
            "banjar": {"kata": "salawat jalan", "contoh": "Salawat jalan gasan pian nang handak tulak jauh."},
            "lampung": {"kata": "selamat lapah", "aksara": "ᰞᰦᰜᰦᰕᰦᰊ ᰜᰦᰎᰦᰝ", "contoh": "Selamat lapah selamat sampai tanoh rantau."},
            "papua": {"kata": "selamat jalan", "contoh": "Selamat jalan sodara, Tuhan berkati perjalananmu."}
        }
    },
    {
        "indonesia": "selamat datang", "kelas_kata": "ungkapan", "kategori": "ungkapan_sapaan",
        "daerah": {
            "jawa": {"ngoko": "sugeng rawuh", "krama_inggil": "sugeng rawuh", "aksara": "ꦱꦸꦒꦼꦁ ꦫꦮꦸꦃ", "contoh": "Sugeng rawuh wonten ing Tanah Jawa ingkang asri."},
            "sunda": {"loma": "wilujeng sumping", "lemes_batur": "wilujeng sumping", "aksara": "ᮝᮤᮜᮥᮏᮨᮀ ᮞᮥᮙ᮪ᮕᮤᮀ", "contoh": "Wilujeng sumping di Bumi Parahyangan tanah Pasundan."},
            "bali": {"andap": "rahajeng rauh", "alus_singgih": "rahajeng rauh", "aksara": "ᬭꭇꬰꭇ꬚ꭇꬾꭇꬲ ᬭꭇꬅꭇꬸꭇꬄ", "contoh": "Rahajeng rauh semeton ring Pulau Dewata Bali."},
            "minang": {"kata": "salamaik datang", "contoh": "Salamaik datang di Ranah Minang nan tacinto."},
            "bugis": {"kata": "salama' pole", "aksara": "ᨔᨒᨆ ᨄᨚᨒᨙ", "contoh": "Salama' pole ri tana Ugi masagena."},
            "makassar": {"kata": "salama' battu", "contoh": "Salama' battu ri Butta Gowa Mangkasara'."},
            "batak_toba": {"kata": "horas ma ro", "aksara": "ᯂᯬᯒꯥᯘ ᯔ ᯒᯬ", "contoh": "Horas ma ro hamu tu Tano Batak na uli."},
            "aceh": {"kata": "saleum troh", "contoh": "Saleum troh teuka bak Bumoe Seuramoe Meukah."},
            "banjar": {"kata": "salawat datang", "contoh": "Salawat datang di Bumi Lambung Mangkurat Banjar."},
            "lampung": {"kata": "selamat ghatei", "aksara": "ᰞᰦᰜᰦᰕᰦᰊ ᰎᰦᰊᰦᰤ", "contoh": "Selamat ghatei di Bumi Ruwa Jurai Lampung."},
            "papua": {"kata": "selamat datang", "contoh": "Selamat datang di Tanah Papua, kitorang samua basaudara."}
        }
    }
]

dataset["total_entri"] = len(entries)
dataset["entri"] = entries

output_path = "/home/devops/widya-lang/kurikulum_bahasa_daerah/kamus_bahasa_daerah/database_kamus_nusantara.json"
with open(output_path, "w", encoding="utf-8") as f:
    json.dump(dataset, f, ensure_ascii=False, indent=2)

print(f"Successfully generated Kamus Nusantara JSON with {len(entries)} entries at {output_path}")
