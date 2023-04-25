use crate::cfg::{
    map_to_opt_vec, new_opt_str, some_path_buf,
    user::{
        AliasMap, ArchAttr, CodeNameAttr, LongAliasMap, OsMap, ProfileAttr,
        ProfileDir, ProfileMap, UserCfg, VariantAttr,
    },
    Sample,
};
use get_lxc_core::as_tiny_str;

fn to_alias_map(arr_map: &[(&str, &str)]) -> AliasMap {
    // let size = std::mem::size_of::<OsName>();
    AliasMap::from_iter(
        arr_map
            .iter()
            .map(|(k, v)| (k.to_string(), as_tiny_str(v))),
    )
}

fn to_long_alias_map(arr_map: &[(&str, &str)]) -> LongAliasMap {
    LongAliasMap::from_iter(
        arr_map
            .iter()
            .map(|(k, v)| (k.to_string(), as_tiny_str(v))),
    )
}

impl Sample for UserCfg {
    fn sample() -> Self {
        Self {
            cmt: new_cmt(),
            os: new_os_map(),
            arch: new_arch_attr(),
            codename: new_codename_attr(),
            variant: new_variant_attr(),
            profile: new_profile(),
        }
    }
}

pub(crate) fn new_cmt() -> String {
    "hello world".to_string()
}

fn new_profile() -> ProfileMap {
    let dir = ProfileDir {
        cmt: new_opt_str(""),
        dl: some_path_buf(),
        dl_path: map_to_opt_vec(&[""]),
    };

    let dft = ProfileAttr {
        cmt: new_opt_str(
            r#"This is the default profile
            mirror: If empty, then mirror is selected according to your region.
            src: To be precise, this is index-src, you can use gh, jh, sa"#,
        ),
        inherits: new_opt_str(""),
        src: new_opt_str(""),
        mirror: new_opt_str(""),
        dir,
        img: None,
    };

    let arr = [("default", dft)];

    ProfileMap::from_iter(arr.map(|(s, d)| (s.to_owned(), d)))
}
fn new_os_map() -> OsMap {
    const MAP: [(&str, &str); 20] = [
        ("uuu", "ubuntu"),
        ("u", "ubuntu"),
        ("deb", "debian"),
        ("d", "debian"),
        ("arch", "archlinux"),
        ("a", "archlinux"),
        ("c", "centos"),
        ("f", "fedora"),
        ("ap", "alpine"),
        ("amz", "amazonlinux"),
        ("apt", "apertis"),
        ("el", "openeuler"),
        ("suse", "opensuse"),
        ("owr", "openwrt"),
        ("rl", "rockylinux"),
        ("alma", "almalinux"),
        ("void", "voidlinux"),
        ("spring", "springdalelinux"),
        ("k", "kali"),
        ("😎", "kali"),
    ];
    OsMap {
        cmt: Some("This is a note about operating system aliases".to_owned()),
        alias: to_alias_map(&MAP),
    }
}

fn new_arch_attr() -> ArchAttr {
    const MAP: [(&str, &str); 12] = [
        ("x64", "amd64"),
        ("x86", "i386"),
        ("i686", "i386"),
        ("i586", "i386"),
        ("rv64", "riscv64"),
        ("aarch64", "arm64"),
        ("a64", "arm64"),
        ("arm", "armhf"),
        ("armv5", "armel"),
        ("ppc", "ppc64el"),
        ("mips", "mipsel"),
        ("m64", "mips64el"),
    ];

    ArchAttr {
        cmt: None,
        alias: to_alias_map(&MAP),
    }
}

fn new_codename_attr() -> CodeNameAttr {
    const MAP: [(&str, &str); 11] = [
        ("d11", "bullseye"),
        ("d12", "bookworm"),
        ("d13", "trixie"),
        ("d14", "forky"),
        ("u20", "focal"),
        ("u22", "jammy"),
        ("u2210", "kinetic"),
        ("u2304", "lunar"),
        ("raw", "rawhide"),
        ("tw", "tumbleweed"),
        ("9s", "9-Stream"),
    ];

    CodeNameAttr {
        cmt: None,
        alias: to_long_alias_map(&MAP),
    }
}
fn new_variant_attr() -> VariantAttr {
    const MAP: [(&str, &str); 4] = [
        ("dft", "default"),
        ("d", "default"),
        ("cld", "cloud"),
        ("m", "musl"),
    ];

    VariantAttr {
        cmt: None,
        alias: to_alias_map(&MAP),
    }
}
