Feel free to open PRs if you want to implement a feature.  I also take
patches per E-Mail if you prefer that method.

## Notes:

Please respect the code style I'm using.  Most importantly, do keep in
mind that this library is meant to be fully asynchronous.  This leads to
one very important rule: **All blocking calls MUST be fringe calls**. They
must never be used in any other call inside the library.  In fact, if your
synchronous call is anything more than `<asyncronous future stack>.wait()` have
a good reason for it.
