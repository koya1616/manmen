import { useTranslation } from "react-i18next";
import type { ArgumentDef, ArgumentType } from "../../types";

interface DynamicFormProps {
  arguments: ArgumentDef[];
  values: Record<string, unknown>;
  onChange: (key: string, value: unknown) => void;
  errors?: Record<string, string>;
}

export function DynamicForm({
  arguments: argDefs,
  values,
  onChange,
  errors = {},
}: DynamicFormProps) {
  return (
    <div className="dynamic-form">
      {argDefs.map((arg) => (
        <FormField
          key={arg.id}
          argument={arg}
          value={values[arg.id]}
          onChange={(value) => onChange(arg.id, value)}
          error={errors[arg.id]}
        />
      ))}
    </div>
  );
}

interface FormFieldProps {
  argument: ArgumentDef;
  value: unknown;
  onChange: (value: unknown) => void;
  error?: string;
}

function FormField({ argument, value, onChange, error }: FormFieldProps) {
  const { t } = useTranslation();
  const inputId = `arg-${argument.id}`;
  const unitKey = argument.unit ? `units.${argument.unit}` : null;
  const unitLabel = unitKey ? t(unitKey, { defaultValue: argument.unit ?? '' }) : null;

  return (
    <div className={`form-field ${error ? "has-error" : ""}`}>
      <label htmlFor={inputId}>
        {argument.name}
        {argument.required && <span className="required">*</span>}
      </label>
      {argument.description && (
        <p className="field-description">{argument.description}</p>
      )}
      <InputControl
        id={inputId}
        type={argument.type}
        value={value}
        argument={argument}
        onChange={onChange}
        unitLabel={unitLabel}
      />
      {error && <p className="field-error">{error}</p>}
    </div>
  );
}

interface InputControlProps {
  id: string;
  type: ArgumentType;
  value: unknown;
  argument: ArgumentDef;
  onChange: (value: unknown) => void;
  unitLabel: string | null;
}

function InputControl({ id, type, value, argument, onChange, unitLabel }: InputControlProps) {
  switch (type) {
    case "boolean":
      return (
        <label className="switch-label">
          <input
            id={id}
            type="checkbox"
            checked={!!value}
            onChange={(e) => onChange(e.target.checked)}
          />
          <span className="switch-text">{value ? "ON" : "OFF"}</span>
        </label>
      );

    case "integer":
    case "number":
      return (
        <div className="number-input-group">
          <input
            id={id}
            type="number"
            value={value !== undefined && value !== null ? String(value) : ""}
            min={argument.min}
            max={argument.max}
            onChange={(e) => {
              const num = type === "integer" ? parseInt(e.target.value, 10) : parseFloat(e.target.value);
              onChange(isNaN(num) ? null : num);
            }}
            placeholder={argument.default !== null && argument.default !== undefined ? String(argument.default) : ""}
          />
          {unitLabel && <span className="unit">{unitLabel}</span>}
        </div>
      );

    case "enum":
      return (
        <select
          id={id}
          value={value !== undefined && value !== null ? String(value) : ""}
          onChange={(e) => onChange(e.target.value || null)}
        >
          <option value="">Select...</option>
          {argument.options?.map((opt) => (
            <option key={opt} value={opt}>
              {opt}
            </option>
          ))}
        </select>
      );

    case "duration":
      return (
        <div className="number-input-group">
          <input
            id={id}
            type="number"
            value={value !== undefined && value !== null ? String(value) : ""}
            min={argument.min ?? 0}
            onChange={(e) => {
              const num = parseFloat(e.target.value);
              onChange(isNaN(num) ? null : num);
            }}
            placeholder={argument.default !== null && argument.default !== undefined ? String(argument.default) : ""}
          />
          <span className="unit">{unitLabel || argument.unit || "seconds"}</span>
        </div>
      );

    case "path":
    case "file":
    case "directory":
      return (
        <input
          id={id}
          type="text"
          value={value !== undefined && value !== null ? String(value) : ""}
          onChange={(e) => onChange(e.target.value || null)}
          placeholder={`Enter ${type}...`}
        />
      );

    case "string":
    default:
      return (
        <input
          id={id}
          type="text"
          value={value !== undefined && value !== null ? String(value) : ""}
          onChange={(e) => onChange(e.target.value || null)}
          placeholder={argument.default !== null && argument.default !== undefined ? String(argument.default) : ""}
        />
      );
  }
}
