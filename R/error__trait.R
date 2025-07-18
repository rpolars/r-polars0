# ANY NEW ERROR MUST IMPLEMENT THESE S3 METHODS, these are the "trait" of a polars error
# ALSO MUST IMPLEMENT THESE BASE METHODS: print

#' Internal generic method to add call to error
#' @param err any type which impl as.character
#' @param call calling context
#' @noRd
#' @details
#' Additional details...
#'
#' @return err as string
when_calling = function(err, call) {
  if (polars_options()$do_not_repeat_call || is.null(call)) {
    err
  } else {
    UseMethod("when_calling", err)
  }
}

#' @exportS3Method
when_calling.default = function(err, call) {
  stop("internal error: an error-type was not fully implemented")
}
# support function to convert a call to a string
call_to_string = function(call) paste(capture.output(print(call)), collapse = "\n")
# NB collapse is needed to ensure no invalid multi-line error strings


#' where in (lexically) error happened
#' @description Internal generic method to point to which public method the user got wrong
#' @param err any type which impl as.character
#' @param context calling context
#' @noRd
#' @return err as string
where_in = function(err, context) {
  if (is.null(context)) {
    return(err)
  }
  if (!is_string(context)) {
    stop(
      paste(
        "internal error: where_in context must be a string or NULL it was: ",
        str_string(context)
      )
    )
  }
  UseMethod("where_in", err)
}

#' @exportS3Method
where_in.default = function(err, context) {
  stop("internal error: an error-type was not fully implemented")
}

#' Internal generic method to convert an error_type to condition.
#' @param err any type which impl as.character
#' @noRd
#' @details
#' this method is needed to preserve state of err without upcasting to a string message
#' an implementation will describe how to store the error in the condition
#' @return condition
to_condition = function(err) {
  UseMethod("to_condition", err)
}

#' @exportS3Method
to_condition.default = function(err) {
  errorCondition(
    paste(capture.output(print(err)), collapse = "\n"),
    class = c("default_error"),
    value = err,
    call = NULL
  )
}

#' Internal generic method to add plain text to error message
#' @param err some error type object
#' @param msg string to add
#' @noRd
#' @return condition
plain = function(err, msg) {
  if (is.null(msg)) {
    return(err)
  }
  UseMethod("plain", err)
}

#' @exportS3Method
plain.default = function(err, msg) {
  paste0(msg, ": ", err)
}


#' Internal generic method to add plain text to error message
#' @details
#' polars converts any other error types to RPolarsErr.
#' An error type can choose to implement this to improve the translation.
#' As fall back the error will be deparsed into a string with rust Debug, see rdbg()
#' @param err some error type object
#' @noRd
#' @return condition
upgrade_err = function(err) {
  UseMethod("upgrade_err", err)
}

#' @exportS3Method
upgrade_err.default = function(err) {
  err # no upgrade found pass as is
}

# call upgrade error from internalsnamespace
# error_trait methods are internal and do not work correctly
# when called directly by user e.g. polars0:::upgrade_err(polars0:::RPolarsErr$new())
# calling R from rust via R! but it is  a "user" call in .GlobalEnv
# by calling a package function the parent env is the internal pacakge env.
upgrade_err_internal_ns = \(x) upgrade_err(x)
