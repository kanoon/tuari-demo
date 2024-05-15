import log from "loglevel";

log.setLevel("info"); // Set the logging level (trace, debug, info, warn, error, silent)

// Customizing log prefix
log.getLogger("my-logger").setDefaultLevel("info");

export default log;
