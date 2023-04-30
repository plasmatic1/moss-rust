* `engine/`
    * `preprocessor/`: For preprocessing documents
    * `fingerprint/`: For fingerprinting algorithm
    * `logging.rs`: for perf/info logging utilities
    * `core.rs`: core code that ties together components in a distributed way
* `ui/`
    * `dmoj/`: DMOJ-specific utils, for testing
        * maybe should be judge/dmoj
    * `web/`: Web interface that gives more interactive results
        * includes backend and templates folders
        * Not sure how to best design the architecture here...
    * `console.rs`: Command-line interface
* `test/`
    * `unit/`
    * `suite/`: Contains info for suite-style testing
    * `suitedata/`: Contains link to separate repository for collecting and maintaining datasets
    * `ci/`: CI scripts for gh
        * Not sure how to validate tbh

TODO: how to generate docs?