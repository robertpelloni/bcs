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

        public override string ToString() {
            return Value?.ToString() ?? "Nil";
        }
    }
}
