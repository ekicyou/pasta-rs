use rhai::{Dynamic, Engine, EvalAltResult, FnPtr, ImmutableString, Module, Position};
use std::any::{type_name, Any};

pub type FuncReturn<T> = Result<T, Box<EvalAltResult>>;

pub trait DynamicExt {
    fn cast_result<T: Any + Clone>(self) -> FuncReturn<T>;
}
impl DynamicExt for Dynamic {
    fn cast_result<T: Any + Clone>(self) -> FuncReturn<T> {
        let act = self.type_name().to_owned();
        match self.try_cast::<T>() {
            Some(a) => Ok(a),
            None => {
                let exp = type_name::<T>().to_owned();
                Err(Box::new(EvalAltResult::ErrorMismatchOutputType(
                    exp,
                    act,
                    Position::none(),
                )))
            }
        }
    }
}

pub trait FnPtrExt {
    fn set_result(&mut self, dy: Dynamic) -> FuncReturn<()>;
    fn set(&mut self, dy: Dynamic);
}
impl FnPtrExt for FnPtr {
    fn set_result(&mut self, dy: Dynamic) -> FuncReturn<()> {
        *self = dy.cast_result::<FnPtr>()?;
        Ok(())
    }
    fn set(&mut self, dy: Dynamic) {
        if let Err(e) = self.set_result(dy) {
            log::error!("{}", e);
        }
    }
}

pub trait FnPtrCallImmutableString {
    /// 関数適用
    fn call_result<S: Into<ImmutableString>>(
        &self,
        engine: &Engine,
        lib: &impl AsRef<Module>,
        text: S,
    ) -> FuncReturn<Dynamic>;

    /// action適用（戻り値無し）
    fn call<S: Into<ImmutableString>>(
        &self,
        engine: &Engine,
        lib: &impl AsRef<Module>,
        text: S,
    ) -> FuncReturn<()> {
        self.call_result(engine, lib, text)?;
        Ok(())
    }

    /// str戻り値の関数適用
    fn call_string<S: Into<ImmutableString>>(
        &self,
        engine: &Engine,
        lib: &impl AsRef<Module>,
        text: S,
    ) -> FuncReturn<String> {
        Ok(self.call_result(engine, lib, text)?.take_string()?)
    }
}
impl FnPtrCallImmutableString for FnPtr {
    /// emote 適用
    fn call_result<S: Into<ImmutableString>>(
        &self,
        engine: &Engine,
        lib: &impl AsRef<Module>,
        text: S,
    ) -> FuncReturn<Dynamic> {
        let a1 = Dynamic::from(text.into());
        let dy = self.call_dynamic(engine, lib, None, [a1])?;
        Ok(dy)
    }
}

pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
    Ok(())
}
