import { useEffect, useState } from "react";
import { useT } from "../i18n";
import { formatDisplayDate, parseDisplayDate } from "../lib/dates";

export function DateField({
  id,
  value,
  onChange,
  required = false,
  disabled = false,
}: {
  id: string;
  value: string;
  onChange: (isoDate: string) => void;
  required?: boolean;
  disabled?: boolean;
}) {
  const t = useT();
  const [draft, setDraft] = useState(value ? formatDisplayDate(value) : "");
  const [focused, setFocused] = useState(false);

  useEffect(() => {
    if (!focused) {
      setDraft(value ? formatDisplayDate(value) : "");
    }
  }, [value, focused]);

  return (
    <input
      id={id}
      type="text"
      inputMode="numeric"
      autoComplete="off"
      spellCheck={false}
      placeholder={t("common.datePlaceholder")}
      value={focused ? draft : value ? formatDisplayDate(value) : draft}
      required={required}
      disabled={disabled}
      onFocus={() => setFocused(true)}
      onChange={(event) => {
        const next = event.target.value;
        setDraft(next);
        if (!next.trim()) {
          event.currentTarget.setCustomValidity("");
          onChange("");
          return;
        }
        const parsed = parseDisplayDate(next);
        if (parsed) {
          event.currentTarget.setCustomValidity("");
          onChange(parsed);
          return;
        }
        event.currentTarget.setCustomValidity(t("common.dateInvalid"));
      }}
      onBlur={(event) => {
        setFocused(false);
        if (!draft.trim()) {
          event.currentTarget.setCustomValidity("");
          onChange("");
          setDraft("");
          return;
        }
        const parsed = parseDisplayDate(draft);
        if (parsed) {
          event.currentTarget.setCustomValidity("");
          onChange(parsed);
          setDraft(formatDisplayDate(parsed));
          return;
        }
        if (value) {
          event.currentTarget.setCustomValidity("");
          setDraft(formatDisplayDate(value));
          return;
        }
        event.currentTarget.setCustomValidity(t("common.dateInvalid"));
      }}
    />
  );
}
