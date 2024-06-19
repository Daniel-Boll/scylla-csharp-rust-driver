<h1 align="center">ScyllaDB C#-Rust Driver</h1>

Wrapper around ScyllaDB's rust-driver compatible with Datastax csharp-driver.

## Examples

```cs
var cluster = Cluster.Builder()
  .AddContactPoints("localhost:9042")
  .Build();

var session = await cluster.Connect("system_schema");

var rs = await session.Execute("SELECT * FROM tables LIMIT 2");

foreach (var row in rs) {
  Console.WriteLine(row.GetValue<string>("table_name"));
  Console.WriteLine(row.GetValue<int>("default_time_to_live"));
}
```
