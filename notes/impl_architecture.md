* Core algorithm
    * Single master thread that dispatches workers
        * Needs: way to understand which workers are busy (maybe a queue?)
        * Once it receives data, it uses it to continue the next step of the algorithm
    * Many worker processes
        * Workers may be preprocessing, doing fingerprinting, or computing matches
* Logging
    * Core algorithm may publish logging events to a handler that can receive those events and do updates.  Useful avenues include
        * i.e. console-based way to report performance
        * summarizing reports at the very end of the algorithm
        * sending updates to a websocket so that the backend may hear