using System.Runtime.InteropServices;

public class Cluster {
  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern IntPtr cluster_builder_new();

  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern void cluster_builder_set_contact_points(IntPtr builder, string contactPoints);

  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern IntPtr cluster_connect(IntPtr builder, string keyspace);

  [DllImport("libscylla_wrapper", CallingConvention = CallingConvention.Cdecl)]
  private static extern void free_pointer(IntPtr ptr);

  private IntPtr _clusterPtr;

  private Cluster(IntPtr clusterPtr) {
    _clusterPtr = clusterPtr;
  }

  public static Cluster Builder() {
    IntPtr builderPtr = cluster_builder_new();
    return new Cluster(builderPtr);
  }

  public Cluster AddContactPoints(string contactPoints) {
    cluster_builder_set_contact_points(_clusterPtr, contactPoints);
    return this;
  }

  public Cluster Build() {
    return this;
  }

  public async Task<Session> Connect(string keyspace) {
    IntPtr sessionPtr = await Task.Run(() => cluster_connect(_clusterPtr, keyspace));
    return new Session(sessionPtr);
  }

  ~Cluster() {
    free_pointer(_clusterPtr);
  }
}
