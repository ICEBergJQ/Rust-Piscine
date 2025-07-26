pub fn first_subword(mut s: String) -> String {
      for (i, c) in s.char_indices() {
        if i == 0 {
            continue;
        }

        if c == '_' || c.is_uppercase() {
            s = s[..i].to_string();
            break;
        }
    }
    s
}