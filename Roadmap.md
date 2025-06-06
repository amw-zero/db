Roadmap.md

# Tracing and Trace Testing

Both observability and testing can be done by emitting traces of behavior. Logging traces will help debug the system behavior, but they can also be used in tests if enough information is added to spans.

This pairs nicely with the model-based test which is already creating lots of behaviors.

# Turmoil

Eventually, concurrency and replication will need to be added. When this happens, we should add [turmoil](https://www.google.com/search?q=rust+turmoil&rlz=1C5GCCM_enUS1131US1131&oq=rust+turmoil&gs_lcrp=EgZjaHJvbWUqCQgAEEUYOxiABDIJCAAQRRg7GIAEMg0IARAAGIYDGIAEGIoFMg0IAhAAGIYDGIAEGIoFMg0IAxAAGIYDGIAEGIoFMgYIBBBFGDwyBggFEEUYPDIGCAYQRRhAMgYIBxBFGEDSAQgxMTE4ajBqN6gCALACAA&sourceid=chrome&ie=UTF-8) so that we can still test deterministically.