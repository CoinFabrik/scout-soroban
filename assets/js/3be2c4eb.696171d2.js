"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[6451],{9613:(e,r,t)=>{t.d(r,{Zo:()=>c,kt:()=>m});var o=t(9496);function n(e,r,t){return r in e?Object.defineProperty(e,r,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[r]=t,e}function a(e,r){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);r&&(o=o.filter((function(r){return Object.getOwnPropertyDescriptor(e,r).enumerable}))),t.push.apply(t,o)}return t}function i(e){for(var r=1;r<arguments.length;r++){var t=null!=arguments[r]?arguments[r]:{};r%2?a(Object(t),!0).forEach((function(r){n(e,r,t[r])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):a(Object(t)).forEach((function(r){Object.defineProperty(e,r,Object.getOwnPropertyDescriptor(t,r))}))}return e}function l(e,r){if(null==e)return{};var t,o,n=function(e,r){if(null==e)return{};var t,o,n={},a=Object.keys(e);for(o=0;o<a.length;o++)t=a[o],r.indexOf(t)>=0||(n[t]=e[t]);return n}(e,r);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(o=0;o<a.length;o++)t=a[o],r.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(n[t]=e[t])}return n}var s=o.createContext({}),u=function(e){var r=o.useContext(s),t=r;return e&&(t="function"==typeof e?e(r):i(i({},r),e)),t},c=function(e){var r=u(e.components);return o.createElement(s.Provider,{value:r},e.children)},d="mdxType",f={inlineCode:"code",wrapper:function(e){var r=e.children;return o.createElement(o.Fragment,{},r)}},p=o.forwardRef((function(e,r){var t=e.components,n=e.mdxType,a=e.originalType,s=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),d=u(t),p=n,m=d["".concat(s,".").concat(p)]||d[p]||f[p]||a;return t?o.createElement(m,i(i({ref:r},c),{},{components:t})):o.createElement(m,i({ref:r},c))}));function m(e,r){var t=arguments,n=r&&r.mdxType;if("string"==typeof e||n){var a=t.length,i=new Array(a);i[0]=p;var l={};for(var s in r)hasOwnProperty.call(r,s)&&(l[s]=r[s]);l.originalType=e,l[d]="string"==typeof e?e:n,i[1]=l;for(var u=2;u<a;u++)i[u]=t[u];return o.createElement.apply(null,i)}return o.createElement.apply(null,t)}p.displayName="MDXCreateElement"},7478:(e,r,t)=>{t.r(r),t.d(r,{assets:()=>s,contentTitle:()=>i,default:()=>f,frontMatter:()=>a,metadata:()=>l,toc:()=>u});var o=t(2564),n=(t(9496),t(9613));const a={},i="Integer overflow or underflow",l={unversionedId:"detectors/integer-overflow -or-underflow",id:"detectors/integer-overflow -or-underflow",title:"Integer overflow or underflow",description:"Description",source:"@site/docs/detectors/21-integer-overflow -or-underflow.md",sourceDirName:"detectors",slug:"/detectors/integer-overflow -or-underflow",permalink:"/scout-soroban/docs/detectors/integer-overflow -or-underflow",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/21-integer-overflow -or-underflow.md",tags:[],version:"current",sidebarPosition:21,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Incorrect exponentiation",permalink:"/scout-soroban/docs/detectors/incorrect-exponentiation"},next:{title:"Storage change events",permalink:"/scout-soroban/docs/detectors/storage-change-events"}},s={},u=[{value:"Description",id:"description",level:2},{value:"Why is this bad?",id:"why-is-this-bad",level:2},{value:"Issue example",id:"issue-example",level:2},{value:"Remediated example",id:"remediated-example",level:2},{value:"How is it detected?",id:"how-is-it-detected",level:2}],c={toc:u},d="wrapper";function f(e){let{components:r,...t}=e;return(0,n.kt)(d,(0,o.Z)({},c,t,{components:r,mdxType:"MDXLayout"}),(0,n.kt)("h1",{id:"integer-overflow-or-underflow"},"Integer overflow or underflow"),(0,n.kt)("h2",{id:"description"},"Description"),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},"Category: ",(0,n.kt)("inlineCode",{parentName:"li"},"Arithmetic")),(0,n.kt)("li",{parentName:"ul"},"Severity: ",(0,n.kt)("inlineCode",{parentName:"li"},"Critical")),(0,n.kt)("li",{parentName:"ul"},"Detectors: ",(0,n.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/integer-overflow-or-underflow"},(0,n.kt)("inlineCode",{parentName:"a"},"integer-overflow-or-underflow"))),(0,n.kt)("li",{parentName:"ul"},"Test Cases: ",(0,n.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-1"},(0,n.kt)("inlineCode",{parentName:"a"},"integer-overflow-or-underflow-1")),(0,n.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-2"},(0,n.kt)("inlineCode",{parentName:"a"},"integer-overflow-or-underflow-2")),(0,n.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-3"},(0,n.kt)("inlineCode",{parentName:"a"},"integer-overflow-or-underflow-3")),(0,n.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-4"},(0,n.kt)("inlineCode",{parentName:"a"},"integer-overflow-or-underflow-4")),(0,n.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-5"},(0,n.kt)("inlineCode",{parentName:"a"},"integer-overflow-or-underflow-5")))),(0,n.kt)("p",null,"In Rust, arithmetic operations can result in a value that falls outside the allowed numerical range for a given type. When the result exceeds the maximum value of the range, it's called an overflow, and when it falls below the minimum value of the range, it's called an underflow."),(0,n.kt)("h2",{id:"why-is-this-bad"},"Why is this bad?"),(0,n.kt)("p",null,"If there are arithmetic operations with overflow or underflow problems, and if errors are not handled correctly, incorrect results will be generated, bringing potential problems for the contract. Additionally, these types of errors can allow attackers to drain a contract\u2019s funds or manipulate its logic."),(0,n.kt)("h2",{id:"issue-example"},"Issue example"),(0,n.kt)("p",null,"Consider the following ",(0,n.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,n.kt)("pre",null,(0,n.kt)("code",{parentName:"pre",className:"language-rust"},"\n pub fn add(env: Env, value: u32) {\n        let current: u32 = env.storage().temporary().get(&Self::VALUE).unwrap_or(0);\n        let new_value = current + value;\n        env.storage().temporary().set(&Self::VALUE, &new_value);\n    }\n\n")),(0,n.kt)("p",null,"In this example, an operation is performed on two u32 values without any safeguards against overflow if it occurs."),(0,n.kt)("p",null,"The code example can be found ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-1/vulnerable-example"},"here"),"."),(0,n.kt)("h2",{id:"remediated-example"},"Remediated example"),(0,n.kt)("pre",null,(0,n.kt)("code",{parentName:"pre",className:"language-rust"},"pub fn add(env: Env, value: u32) -> Result<(), Error> {\n        let current: u32 = env.storage().temporary().get(&Self::VALUE).unwrap_or(0);\n        let new_value = match current.checked_add(value) {\n            Some(value) => value,\n            None => return Err(Error::OverflowError),\n        };\n        env.storage().temporary().set(&Self::VALUE, &new_value);\n        Ok(())\n    }       \n")),(0,n.kt)("p",null,"In this example, the ",(0,n.kt)("inlineCode",{parentName:"p"},"checked_add")," method is used to perform the addition. It returns the sum if no overflow occurs; otherwise, it returns ",(0,n.kt)("inlineCode",{parentName:"p"},"None"),", with an OverflowError variant indicating that an overflow error has occurred."),(0,n.kt)("p",null,"The remediated code example can be found ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-1/remediated-example"},"here"),"."),(0,n.kt)("h2",{id:"how-is-it-detected"},"How is it detected?"),(0,n.kt)("p",null,"Checks if there\u2019s any numerical overflow or underflow."))}f.isMDXComponent=!0}}]);