namespace BCS.Core.Tools {
    // BcsVariant acts as a dynamic type wrapper mapping natively to C#'s `object`
    public class BcsVariant {
        public object Value { get; set; }

        public BcsVariant(object value) {
            Value = value;
        }

        public bool IsNil() {
            return Value == null;
        }

        public string GetVariantType() {
            return Value?.GetType().Name ?? "Nil";
        }

        public override string ToString() {
            return Value?.ToString() ?? "Nil";
        }

        public int? ToInt() {
            if (Value is int val) return val;
            return null;
        }

        public double? ToFloat() {
            if (Value is double val) return val;
            return null;
        }

        public bool? ToBool() {
            if (Value is bool val) return val;
            return null;
        }
    }
}
