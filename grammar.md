`Func -> fnc FuncName(OptArgs) OptType { Expr }`

`OptArgs -> Args | Epsilon`
`Args -> Args, Args | Arg`
`Arg -> Name : Type`

`OptType -> -> Type | Epsilon` 

`InfixFunc -> + | - | * | / | FuncName`

`Expr -> FuncName(OptFuncArgs)
       | Expr InfixFunc Expr 
       | (Expr) | (Expr, Expr) 
       | Var 
       | Number`

`OptFuncArgs -> Exprs | Epsilon`
`Exprs -> Exprs, Expr | Expr`


`Declare -> Var := Expr;`
`FuncName -> String`
`Var -> String`

`Number -> i64`
