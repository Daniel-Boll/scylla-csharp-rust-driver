using System.Runtime.InteropServices;

public class Session {
  private IntPtr _sessionPtr;

  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern IntPtr session_execute(IntPtr session, string query);

  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern void free_pointer(IntPtr ptr);

  internal Session(IntPtr sessionPtr) {
    _sessionPtr = sessionPtr;
  }

  public async Task<RowSet> Execute(string query) {
    IntPtr resultPtr = await Task.Run(() => session_execute(_sessionPtr, query));
    return new RowSet(resultPtr);
  }

  ~Session() {
    free_pointer(_sessionPtr);
  }
}
