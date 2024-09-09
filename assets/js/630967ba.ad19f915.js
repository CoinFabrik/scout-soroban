"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[7474],{9613:(e,t,n)=>{n.d(t,{Zo:()=>s,kt:()=>f});var r=n(9496);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var p=r.createContext({}),l=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},s=function(e){var t=l(e.components);return r.createElement(p.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,p=e.parentName,s=c(e,["components","mdxType","originalType","parentName"]),u=l(n),m=o,f=u["".concat(p,".").concat(m)]||u[m]||d[m]||a;return n?r.createElement(f,i(i({ref:t},s),{},{components:n})):r.createElement(f,i({ref:t},s))}));function f(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,i=new Array(a);i[0]=m;var c={};for(var p in t)hasOwnProperty.call(t,p)&&(c[p]=t[p]);c.originalType=e,c[u]="string"==typeof e?e:o,i[1]=c;for(var l=2;l<a;l++)i[l]=n[l];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},6990:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>i,default:()=>d,frontMatter:()=>a,metadata:()=>c,toc:()=>l});var r=n(2564),o=(n(9496),n(9613));const a={},i="Incorrect exponentiation",c={unversionedId:"detectors/incorrect-exponentiation",id:"detectors/incorrect-exponentiation",title:"Incorrect exponentiation",description:"Description",source:"@site/docs/detectors/20-incorrect-exponentiation.md",sourceDirName:"detectors",slug:"/detectors/incorrect-exponentiation",permalink:"/scout-soroban/docs/detectors/incorrect-exponentiation",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/20-incorrect-exponentiation.md",tags:[],version:"current",sidebarPosition:20,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Unsafe map get",permalink:"/scout-soroban/docs/detectors/unsafe-map-get"},next:{title:"Integer overflow or underflow",permalink:"/scout-soroban/docs/detectors/integer-overflow -or-underflow"}},p={},l=[{value:"Description",id:"description",level:2},{value:"Why is it bad?",id:"why-is-it-bad",level:2},{value:"Issue example",id:"issue-example",level:2},{value:"Remediated example",id:"remediated-example",level:2},{value:"How is it detected?",id:"how-is-it-detected",level:2},{value:"References",id:"references",level:2}],s={toc:l},u="wrapper";function d(e){let{components:t,...n}=e;return(0,o.kt)(u,(0,r.Z)({},s,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"incorrect-exponentiation"},"Incorrect exponentiation"),(0,o.kt)("h2",{id:"description"},"Description"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,o.kt)("inlineCode",{parentName:"li"},"Arithmetic")),(0,o.kt)("li",{parentName:"ul"},"Vulnerability Severity: ",(0,o.kt)("inlineCode",{parentName:"li"},"Critical")),(0,o.kt)("li",{parentName:"ul"},"Detectors: ",(0,o.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/incorrect-exponentiation"},(0,o.kt)("inlineCode",{parentName:"a"},"incorrect-exponentiation"))),(0,o.kt)("li",{parentName:"ul"},"Test Cases: ",(0,o.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/incorrect-exponentiation/incorrect-exponentiation-1"},(0,o.kt)("inlineCode",{parentName:"a"},"incorrect-exponentiation-1")))),(0,o.kt)("p",null,"The operator ",(0,o.kt)("inlineCode",{parentName:"p"},"^")," is not an exponential operator, it is a bitwise XOR. Make sure to use ",(0,o.kt)("inlineCode",{parentName:"p"},"pow()")," instead for exponentiation. In case of performing a XOR operation, use ",(0,o.kt)("inlineCode",{parentName:"p"},".bitxor()")," for clarity."),(0,o.kt)("h2",{id:"why-is-it-bad"},"Why is it bad?"),(0,o.kt)("p",null,"It can produce unexpected behaviour in the smart contract."),(0,o.kt)("h2",{id:"issue-example"},"Issue example"),(0,o.kt)("p",null,"In the following example, the ",(0,o.kt)("inlineCode",{parentName:"p"},"^")," operand is being used for exponentiation. But in Rust, ",(0,o.kt)("inlineCode",{parentName:"p"},"^")," is the operand for an XOR operation. If misused, this could lead to unexpected behaviour in our contract."),(0,o.kt)("p",null,"Consider the following ",(0,o.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'   pub fn exp_data_3(e: Env) -> u128 {\n        let mut data = e.storage()\n        .instance()\n        .get::<DataKey, u128>(&DataKey::Data)\n        .expect("Data not found");\n        \n        data ^= 3;\n        data\n    }\n')),(0,o.kt)("p",null,"The code example can be found ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/incorrect-exponentiation/incorrect-exponentiation-1/vulnerable-example"},"here"),"."),(0,o.kt)("h2",{id:"remediated-example"},"Remediated example"),(0,o.kt)("p",null,"A possible solution is to use the method ",(0,o.kt)("inlineCode",{parentName:"p"},"pow()"),". But, if a XOR operation is wanted, ",(0,o.kt)("inlineCode",{parentName:"p"},".bitxor()")," method is recommended."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'    pub fn exp_data_3(e: Env) -> u128 {\n        let data = e.storage()\n        .instance()\n        .get::<DataKey, u128>(&DataKey::Data)\n        .expect("Data not found");\n\n        data.pow(3)\n    }\n')),(0,o.kt)("p",null,"The remediated code example can be found ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/incorrect-exponentiation/incorrect-exponentiation-1/remediated-example"},"here"),"."),(0,o.kt)("h2",{id:"how-is-it-detected"},"How is it detected?"),(0,o.kt)("p",null,"Warns about ",(0,o.kt)("inlineCode",{parentName:"p"},"^")," being a ",(0,o.kt)("inlineCode",{parentName:"p"},"bit XOR")," operation instead of an exponentiation.  "),(0,o.kt)("h2",{id:"references"},"References"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("a",{parentName:"li",href:"https://doc.rust-lang.org/std/ops/trait.BitXor.html"},"https://doc.rust-lang.org/std/ops/trait.BitXor.html"))))}d.isMDXComponent=!0}}]);