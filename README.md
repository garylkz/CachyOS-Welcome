<div align="center">
  <h1>CachyOS Welcome</h1>
  <p>
    <strong>Welcome screen for CachyOS written in Rust</strong>
  </p>
  <p>

[![Dependency Status](https://deps.rs/repo/github/cachyos/cachyos-welcome/status.svg)](https://deps.rs/repo/github/cachyos/cachyos-welcome)
<br />
[![CI](https://github.com/cachyos/cachyos-welcome/actions/workflows/rust.yml/badge.svg)](https://github.com/cachyos/cachyos-welcome/actions/workflows/rust.yml)

  </p>
</div>

## Translation Guidelines for CachyOS-Welcome

We are always open to community members adding new translations. This helps immensely for non-English speakers.

### Directory Structure in `i18n`

```md
CachyOS-Welcome/i18n/en # Main English directory with all original text.
CachyOS-Welcome/i18n/de # German translation directory.
CachyOS-Welcome/i18n/cs # Czech translation directory.
CachyOS-Welcome/i18n/_your_language_code # Directory for your language translation.
```

### Steps for Adding a Translation

1. **Determine Your Language Code**  
   Refer to the [ISO 639-1 list of language codes](https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes#Table) to find the proper code for your language.
   
2. **Locate the `i18n` Directory**  
   This directory, available in the repository, contains all existing translations.

3. **Create a Directory for Your Language**  
   Use the code of your language (e.g., `fr` for French, `es` for Spanish) as the directory name.

4. **Use the English Files as Templates**  
   Copy the files from `CachyOS-Welcome/i18n/en` to your newly created directory. Translate the content from **English** into **your language**, ensuring the translations are meaningful and accurate.

5. **Submit a Pull Request (PR)**  
   Once your translations are complete, submit a Pull Request (PR) to the repository. This allows us to review and merge your contribution.
