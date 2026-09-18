# WSM Target Contract / Цільовий контракт WSM

`wsm-target-contract` є нейтральним, машинно-читаним контрактом ABI для
першої x86_64-цілі WSM. Його читають два рівноправні споживачі:

```text
CML — емітує код, який зобов'язаний дотримуватися контракту
wsm-os-lisp — надає рантайм і платформове виконання цього контракту
```

Репозиторій навмисно не є ні компілятором, ні рантаймом. Він визначає лише
спільну межу: слово й теги значень, вирівнювання `cons` і closure-дескриптора,
ABI виклику x86_64 та імена дозволених runtime imports. Це розриває колишню
циклічну залежність `CML → wsm-os-lisp → CML`.

The repository is deliberately neither a compiler nor a runtime. It defines
only their shared target boundary: value words/tags, `cons` and closure
alignment, the x86_64 calling ABI, and allowed runtime imports.

## Структура / Structure

- `wsm-os-target/` — Rust `no_std` package `wsm-os-target`; числове джерело
  істини і перевірки його меж.
- `target-contract.lisp` — згенерована Lisp-readable WSM-проєкція тих самих чисел. Її не
  редагують вручну; тест пакета вимагає байт-точної відповідності.

## Межі авторитету / Authority boundaries

`my-lisp` визначає семантику мови. CML визначає parsing, admission, IR та
емісію. `wsm-os-lisp` визначає heap, boot і платформову поведінку. Цей package
не переносить жодну з цих відповідальностей: він лише фіксує їхній спільний
машинний інтерфейс.

## Перевірка / Verification

```bash
cargo test -p wsm-os-target
```

Ліцензія авторської роботи — [ВОЛЬНІСТЬ](LICENSE).
