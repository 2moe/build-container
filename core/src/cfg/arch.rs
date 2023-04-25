use crate::cfg::ArchKey;

pub const fn set_arch_key(key: &str) -> ArchKey {
    match ArchKey::from_str(key) {
        Ok(x) => x,
        _ => panic!("Const panic! Failed to parse &str to Architecture Key."),
    }
}

/// Generates an array of ArchKey from a list of string literals.
/// It allows the user to specify a comma-separated list of architecture keys and
/// converts each key to an ArchKey by calling the set_arch_key function for each key.
///
/// It is equivalent to `["element"].map(|x| set_arch_key(x))`, except that this macro can be called within `const fn`, whereas `arr.map()` cannot.
macro_rules! arch_list {
    // This branch matches when the list has at least one element, with a trailing comma
    ($($k:literal,)+) => {
        [$(set_arch_key($k)),*]
    };
    // This branch matches when the list has zero or more elements, without a trailing comma
    ($($k:literal),*) => {
        [$(set_arch_key($k)),*]
    };
}

pub const fn deb_arch_list() -> [ArchKey; 10] {
    arch_list![
        "amd64", "arm64", "armel", "armhf", "i386", "mips64el", "mipsel", "ppc64el",
        "riscv64", "s390x",
    ]
}

const fn get_deb_arch() -> &'static str {
    match () {
        #[cfg(target_arch = "x86_64")]
        () => "amd64",

        #[cfg(target_arch = "aarch64")]
        () => "arm64",

        #[cfg(target_arch = "riscv64")]
        () => "riscv64",

        #[cfg(all(target_arch = "arm", target_feature = "vfpv3"))]
        () => "armhf",

        #[cfg(all(target_arch = "arm", not(target_feature = "vfpv3")))]
        () => "armel",

        #[cfg(all(target_arch = "mips", target_endian = "little"))]
        () => "mipsel",

        #[cfg(all(target_arch = "mips64", target_endian = "little"))]
        () => "mips64el",

        #[cfg(target_arch = "s390x")]
        () => "s390x",

        #[cfg(all(target_arch = "powerpc64", target_endian = "little"))]
        () => "ppc64el",

        #[cfg(target_arch = "x86")]
        () => "i386",

        #[allow(unreachable_patterns)]
        _ => "amd64",
    }
}

pub const fn arch_key_default() -> (&'static str, ArchKey) {
    let arch = get_deb_arch();
    match ArchKey::from_str(arch) {
        Ok(a) => (arch, a),
        _ => panic!(
            "Const panic! The available value are x86_64, aarch64, riscv64, etc.",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_arch() {
        dbg!(arch_key_default());

        #[cfg(target_endian = "little")]
        println!("LE");

        #[cfg(target_endian = "big")]
        println!("BE");
    }

    #[test]
    fn print_deb_arch_list() {
        dbg!(deb_arch_list());
    }
}
