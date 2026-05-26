# TLCA l10n

## Generating the PO Template

```sh
MDBOOK_OUTPUT='{"xgettext": {}}' \
  mdbook build -d po
```

## Initialize a New Translation

```sh
msginit -i po/messages.pot -l ru -o po/ru.po --no-translator
```

## Updating an Existing Translation

```sh
msgmerge --update po/ru.po po/messages.pot
```

```sh
msgcat --no-wrap po/ru.po -o po/ru.po
```

```sh
msgcat po/ru.po -o po/ru.po
```

```sh
msgfmt --statistics po/ru.po
```

## Build and postprocess all translations

```sh
for language in en ru; do
    echo "Start building $language translation"
    MDBOOK_BOOK__LANGUAGE=$language \
    mdbook build -d book/$language
    echo "End building"
done
```

---


## Other

`s/<!-- i18n:skip -->\s*\n//g;`

`msgid "\[.*\]\(.*\)"`

```
MDBOOK_BOOK__LANGUAGE=ru mdbook build
```

```
MDBOOK_BOOK__LANGUAGE=ru mdbook serve
```
