// Permitir a nivel crate la existencia de código
// no utilizado sin emitir el warning al compilar
#![allow(dead_code)]

mod geom;
mod road;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
