using System.Collections;
using System.Runtime.InteropServices;

public class RowSet : IEnumerable<Row> {
  private IntPtr _resultPtr;
  private ScyllaRow _rows;

  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern ScyllaRow row_set_get_rows(IntPtr rs);

  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern void free_pointer(IntPtr ptr);

  internal RowSet(IntPtr resultPtr) {
    _resultPtr = resultPtr;
    _rows = row_set_get_rows(resultPtr);
  }

  public IEnumerator<Row> GetEnumerator() {
    for (int i = 0; i < _rows.len; i++) {
      yield return new Row(_resultPtr, i);
    }
  }

  IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

  ~RowSet() {
    free_pointer(_resultPtr);
  }
}

public class Row {
  private IntPtr _resultPtr;
  private int _index;

  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern int row_get_value_int(IntPtr row, int index, string column);
  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern IntPtr row_get_value_string(IntPtr row, int index, string column);
  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern void free_pointer(IntPtr ptr);

  internal Row(IntPtr resultPtr, int index) {
    _resultPtr = resultPtr;
    _index = index;
  }

  public T GetValue<T>(string column) {
    if (typeof(T) == typeof(int)) {
      int value = row_get_value_int(_resultPtr, _index, column);
      return (T)(object)value;
    } else if (typeof(T) == typeof(string)) {
      IntPtr value = row_get_value_string(_resultPtr, _index, column);
      return (T)(object)Marshal.PtrToStringAnsi(value);
    }
    else {
      throw new NotSupportedException($"Type {typeof(T)} is not supported");
    }
  }
}

[StructLayout(LayoutKind.Sequential)]
public struct ScyllaRow {
  public int len;
}
