(function() {var implementors = {};
implementors["getrandom"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["rand"] = [{"text":"impl Error for BernoulliError","synthetic":false,"types":[]},{"text":"impl Error for WeightedError","synthetic":false,"types":[]},{"text":"impl Error for ReadError","synthetic":false,"types":[]}];
implementors["rand_core"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["widestring"] = [{"text":"impl&lt;C:&nbsp;UChar&gt; Error for MissingNulError&lt;C&gt;","synthetic":false,"types":[]},{"text":"impl&lt;C:&nbsp;UChar&gt; Error for NulError&lt;C&gt;","synthetic":false,"types":[]},{"text":"impl Error for FromUtf32Error","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()