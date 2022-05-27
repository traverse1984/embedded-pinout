#[macro_export]
macro_rules! pinout {
   () => {};

   (
       input pulldown { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
      $(let $name = $pin.into_pull_down_input();)*
      pinout!($($tail)*)
   };

   (
       input pullup { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
       $(let $name = $pin.into_pull_up_input();)*
       pinout!($($tail)*)
   };

   (
       input floating { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
       $(let $name = $pin.into_floating_input();)*
       pinout!($($tail)*)
   };

   (
       output { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
      $(let mut $name = $pin.into_push_pull_output();)*
      pinout!($($tail)*)
   };

   (
       output low { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
       $(
           let mut $name = {
               let mut pin = $pin.into_push_pull_output();
               pin.set_low().ok().unwrap();
               pin
           };
       )*
       pinout!($($tail)*)
   };

   (
       output high { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
       $(
           let mut $name = {
               let mut pin = $pin.into_push_pull_output();
               pin.set_high().ok().unwrap();
               pin
           };
       )*
       pinout!($($tail)*)
   };

   (
      disabled floating { $($pin: expr),* $(,)? }
      $($tail: tt)*
   ) => {
      $($pin.into_floating_disabled();)*
      pinout!($($tail)*)
   };

   (
      disabled pulldown { $($pin: expr),* $(,)? }
      $($tail: tt)*
   ) => {
      $($pin.into_pull_down_disabled();)*
      pinout!($($tail)*)
   };

   (
      disabled pullup { $($pin: expr),* $(,)? }
      $($tail: tt)*
   ) => {
      $($pin.into_pull_up_disabled();)*
      pinout!($($tail)*)
   };

   (
       spi { $($pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
      $(
          let _ = $pin.into_mode::<rp2040_hal::gpio::FunctionSpi>();
       )*
       pinout!($($tail)*)
   };

}

mod tests {

    #[test]
    fn test() {
        use Mode::*;
        //  #[allow(unstable_features)]
        //  trace_macros!(true);

        let mut s1 = Pin::new();
        let mut s2 = Pin::new();
        let mut dd1 = Pin::new();
        let mut dd2 = Pin::new();
        let mut du1 = Pin::new();
        let mut du2 = Pin::new();
        let mut df1 = Pin::new();
        let mut df2 = Pin::new();

        pinout!(
           input pulldown { id1 = Pin::new(), id2 = Pin::new() }
           input pullup { iu1 = Pin::new(), iu2 = Pin::new() }
           input floating { if1 = Pin::new(), if2 = Pin::new() }
           output { o1 = Pin::new(), o2 = Pin::new() }
           output low { ol1 = Pin::new(), ol2 = Pin::new() }
           output high { oh1 = Pin::new(), oh2 = Pin::new() }
           spi { s1, s2 }
           disabled pulldown { dd1, dd2 }
           disabled pullup { du1, du2 }
           disabled floating { df1, df2 }
        );

        assert_eq!((id1.mode, id2.mode), (InputPullDown, InputPullDown));
        assert_eq!((iu1.mode, iu2.mode), (InputPullUp, InputPullUp));
        assert_eq!((if1.mode, if2.mode), (InputFloating, InputFloating));
        assert_eq!((o1.mode, o2.mode), (Output, Output));
        assert_eq!((ol1.mode, ol2.mode), (OutputLow, OutputLow));
        assert_eq!((oh1.mode, oh2.mode), (OutputHigh, OutputHigh));
        assert_eq!((s1.mode, s2.mode), (Spi, Spi));
        assert_eq!((dd1.mode, dd2.mode), (DisabledPullDown, DisabledPullDown));
        assert_eq!((du1.mode, du2.mode), (DisabledPullUp, DisabledPullUp));
        assert_eq!((df1.mode, df2.mode), (DisabledFloating, DisabledFloating));
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Mode {
        InputPullDown,
        InputPullUp,
        InputFloating,
        Output,
        OutputLow,
        OutputHigh,
        DisabledPullDown,
        DisabledPullUp,
        DisabledFloating,
        Spi,
        None,
        Incorrect,
    }

    pub struct Pin {
        mode: Mode,
    }

    impl Pin {
        fn new() -> Self {
            Self { mode: Mode::None }
        }

        fn into_pull_down_input(mut self) -> Self {
            self.mode = Mode::InputPullDown;
            self
        }

        fn into_pull_up_input(mut self) -> Self {
            self.mode = Mode::InputPullUp;
            self
        }

        fn into_floating_input(mut self) -> Self {
            self.mode = Mode::InputFloating;
            self
        }

        fn into_push_pull_output(mut self) -> Self {
            self.mode = Mode::Output;
            self
        }

        fn set_low(&mut self) {
            self.mode = match self.mode {
                Mode::Output => Mode::OutputLow,
                _ => Mode::Incorrect,
            };
        }

        fn set_high(&mut self) {
            self.mode = match self.mode {
                Mode::Output => Mode::OutputHigh,
                _ => Mode::Incorrect,
            };
        }

        fn into_pull_down_disabled(&mut self) -> &mut Self {
            self.mode = Mode::DisabledPullDown;
            self
        }

        fn into_pull_up_disabled(&mut self) -> &mut Self {
            self.mode = Mode::DisabledPullUp;
            self
        }

        fn into_floating_disabled(&mut self) -> &mut Self {
            self.mode = Mode::DisabledFloating;
            self
        }

        fn into_mode<T>(&mut self) -> &mut Self {
            self.mode = Mode::Spi;
            self
        }
    }
}
