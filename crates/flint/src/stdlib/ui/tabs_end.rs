//! `ui.tabs_end` — `dst = html ++ <close panels area + container + inline JS>`.
//! The inline script activates tab switching. The first tab is shown by default.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

const TABS_JS: &str = r#"<script>
(function(){
  var btns=document.querySelectorAll('.flint-tab-btn');
  var panels=document.querySelectorAll('.flint-tab-panel');
  function show(id){
    panels.forEach(function(p){p.style.display=p.id===id?'block':'none';});
    btns.forEach(function(b){
      if(b.getAttribute('data-tab')===id){b.className+=' active';}
      else{b.className=b.className.replace(/\s*active/g,'');}
    });
  }
  btns.forEach(function(b){b.onclick=function(){show(this.getAttribute('data-tab'));};});
  if(btns.length){show(btns[0].getAttribute('data-tab'));}
})();
</script>
"#;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.tabs_end")?, "ui.tabs_end", 0)?;
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}</div></div>{TABS_JS}"
        )))))
    })
}
