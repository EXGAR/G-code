use std::ffi::OsString;

pub(crate) fn rewrite_named_provider_args(
    args: Vec<OsString>,
    is_builtin_provider: impl Fn(&str) -> bool,
    is_named_profile: impl Fn(&str) -> bool,
) -> Vec<OsString> {
    let mut rewritten = Vec::with_capacity(args.len());
    let mut index = 0;

    while index < args.len() {
        let argument = &args[index];
        let Some(argument_text) = argument.to_str() else {
            rewritten.push(argument.clone());
            index += 1;
            continue;
        };

        if argument_text == "--" {
            rewritten.extend(args[index..].iter().cloned());
            break;
        }

        if matches!(argument_text, "--provider" | "-p")
            && let Some(value) = args.get(index + 1)
            && let Some(value_text) = value.to_str()
            && !is_builtin_provider(value_text)
            && is_named_profile(value_text)
        {
            rewritten.push(OsString::from("--provider-profile"));
            rewritten.push(value.clone());
            index += 2;
            continue;
        }

        if let Some(value) = argument_text.strip_prefix("--provider=")
            && !is_builtin_provider(value)
            && is_named_profile(value)
        {
            rewritten.push(OsString::from(format!("--provider-profile={value}")));
            index += 1;
            continue;
        }

        rewritten.push(argument.clone());
        index += 1;
    }

    rewritten
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rewrite(args: &[&str]) -> Vec<String> {
        rewrite_named_provider_args(
            args.iter().map(OsString::from).collect(),
            |value| matches!(value, "auto" | "openai" | "openai-compatible"),
            |value| matches!(value, "baizhi" | "other-profile"),
        )
        .into_iter()
        .map(|value| value.into_string().expect("test argument should be UTF-8"))
        .collect()
    }

    #[test]
    fn rewrites_named_profile_after_long_provider_flag() {
        assert_eq!(
            rewrite(&["jcode", "login", "--provider", "baizhi"]),
            ["jcode", "login", "--provider-profile", "baizhi"]
        );
    }

    #[test]
    fn rewrites_named_profile_in_long_equals_form() {
        assert_eq!(
            rewrite(&["jcode", "login", "--provider=baizhi"]),
            ["jcode", "login", "--provider-profile=baizhi"]
        );
    }

    #[test]
    fn rewrites_named_profile_after_short_provider_flag() {
        assert_eq!(
            rewrite(&["jcode", "login", "-p", "baizhi"]),
            ["jcode", "login", "--provider-profile", "baizhi"]
        );
    }

    #[test]
    fn preserves_builtin_provider_values() {
        assert_eq!(
            rewrite(&["jcode", "login", "--provider", "openai"]),
            ["jcode", "login", "--provider", "openai"]
        );
    }

    #[test]
    fn preserves_unknown_provider_values_for_clap_to_reject() {
        assert_eq!(
            rewrite(&["jcode", "login", "--provider", "not-configured"]),
            ["jcode", "login", "--provider", "not-configured"]
        );
    }

    #[test]
    fn preserves_arguments_after_the_option_terminator() {
        assert_eq!(
            rewrite(&["jcode", "run", "--", "--provider", "baizhi"]),
            ["jcode", "run", "--", "--provider", "baizhi"]
        );
    }
}
