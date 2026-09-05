import { useEffect, useRef, useState } from "react";
import { useT } from "../i18n";
import { formatDisplayDate, parseDisplayDate } from "../lib/dates";

function openNativePicker(input: HTMLInputElement) {
  try {
    if (typeof input.showPicker === "function") {
      input.showPicker();
      return;
    }
  } catch {
    // Fall through to a click, which still opens the picker in WebView2.
  }
  input.click();
}

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
  const pickerRef = useRef<HTMLInputElement>(null);
  const [draft, setDraft] = useState(value ? formatDisplayDate(value) : "");
  const [focused, setFocused] = useState(false);

  useEffect(() => {
    if (!focused) {
      setDraft(value ? formatDisplayDate(value) : "");
    }
  }, [value, focused]);

  function applyIso(isoDate: string, input?: HTMLInputElement) {
    input?.setCustomValidity("");
    onChange(isoDate);
    setDraft(isoDate ? formatDisplayDate(isoDate) : "");
  }

  return (
    <div className="date-field">
      <input
        id={id}
        className="date-field-text"
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
            applyIso(parsed, event.currentTarget);
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
      <div className="date-field-picker">
        <input
          ref={pickerRef}
          id={`${id}-picker`}
          className="date-field-native"
          type="date"
          tabIndex={-1}
          aria-hidden="true"
          value={value}
          disabled={disabled}
          onChange={(event) => {
            applyIso(event.target.value);
          }}
        />
        <button
          type="button"
          className="date-field-icon"
          disabled={disabled}
          aria-label={t("common.pickDate")}
          onClick={() => {
            if (pickerRef.current && !disabled) {
              openNativePicker(pickerRef.current);
            }
          }}
        >
          <CalendarIcon />
        </button>
      </div>
    </div>
  );
}

function CalendarIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" aria-hidden="true">
      <rect
        x="2"
        y="3"
        width="12"
        height="11"
        rx="1.5"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.25"
      />
      <path d="M2 6.5h12" fill="none" stroke="currentColor" strokeWidth="1.25" />
      <path d="M5 2v2.5M11 2v2.5" fill="none" stroke="currentColor" strokeWidth="1.25" />
    </svg>
  );
}
