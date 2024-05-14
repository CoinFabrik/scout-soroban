"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[2844],{9613:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var a=n(9496);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function p(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var l=a.createContext({}),s=function(e){var t=a.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},c=function(e){var t=s(e.components);return a.createElement(l.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,l=e.parentName,c=p(e,["components","mdxType","originalType","parentName"]),u=s(n),d=r,f=u["".concat(l,".").concat(d)]||u[d]||m[d]||i;return n?a.createElement(f,o(o({ref:t},c),{},{components:n})):a.createElement(f,o({ref:t},c))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,o=new Array(i);o[0]=d;var p={};for(var l in t)hasOwnProperty.call(t,l)&&(p[l]=t[l]);p.originalType=e,p[u]="string"==typeof e?e:r,o[1]=p;for(var s=2;s<i;s++)o[s]=n[s];return a.createElement.apply(null,o)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},1679:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>o,default:()=>m,frontMatter:()=>i,metadata:()=>p,toc:()=>s});var a=n(2564),r=(n(9496),n(9613));const i={},o="Unsafe map get",p={unversionedId:"vulnerabilities/unsafe-map-get",id:"vulnerabilities/unsafe-map-get",title:"Unsafe map get",description:"Description",source:"@site/docs/vulnerabilities/19-unsafe-map-get.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/unsafe-map-get",permalink:"/scout-soroban/docs/vulnerabilities/unsafe-map-get",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/vulnerabilities/19-unsafe-map-get.md",tags:[],version:"current",sidebarPosition:19,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Unprotected mapping operation",permalink:"/scout-soroban/docs/vulnerabilities/unprotected-mapping-operation"},next:{title:"Incorrect Exponentiation",permalink:"/scout-soroban/docs/vulnerabilities/incorrect-exponentiation"}},l={},s=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Remediation",id:"remediation",level:2}],c={toc:s},u="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"unsafe-map-get"},"Unsafe map get"),(0,r.kt)("h2",{id:"description"},"Description"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,r.kt)("inlineCode",{parentName:"li"},"Validations and error handling")),(0,r.kt)("li",{parentName:"ul"},"Severity: ",(0,r.kt)("inlineCode",{parentName:"li"},"Medium")),(0,r.kt)("li",{parentName:"ul"},"Detectors: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-map-get"},(0,r.kt)("inlineCode",{parentName:"a"},"unsafe-map-get"))),(0,r.kt)("li",{parentName:"ul"},"Test Cases: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-map-get/unsafe-map-get-1"},(0,r.kt)("inlineCode",{parentName:"a"},"unsafe-map-get-1")))),(0,r.kt)("p",null,"The use of certain methods (",(0,r.kt)("inlineCode",{parentName:"p"},"get"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"get_unchecked"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"try_get_unchecked"),") on a ",(0,r.kt)("inlineCode",{parentName:"p"},"Map")," object in the Soroban environment without appropriate error handling can lead to potential runtime panics. This vulnerability stems from accessing the map's values with keys that may not exist, without using safer alternatives that check the existence of the key. Such practices can compromise the robustness of the smart contract by causing it to terminate unexpectedly, which may lead to denial of service or inconsistent state within the contract."),(0,r.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,r.kt)("p",null,"Consider the following ",(0,r.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"    #[contractimpl]\n    impl UnsafeMapGet {\n        pub fn get_from_map(env: Env) -> Option<i32> {\n            let map: Map<Val, Val> = map![&env, (1i32.into_val(&env), 2i64.into_val(&env))];\n            let map: Val = map.into();\n            let map: Map<i32, i32> = map.try_into_val(&env).unwrap();\n            map.get(1)\n        }\n    }\n")),(0,r.kt)("p",null,"This function retrieves values from a map using ",(0,r.kt)("inlineCode",{parentName:"p"},"map.get()")," without checking if the key actually exists in the map. If the key doesn't exist after the conversion, ",(0,r.kt)("inlineCode",{parentName:"p"},"get")," will panic, causing the entire contract to fail."),(0,r.kt)("p",null,"The vulnerable code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-map-get/unsafe-map-get-1/vulnerable-example"},"here"),"."),(0,r.kt)("h2",{id:"remediation"},"Remediation"),(0,r.kt)("p",null,"Both remediated functions presented below ensure the contract doesn't panic due to missing keys. The remediated contract avoid the ",(0,r.kt)("inlineCode",{parentName:"p"},"unsafe map get")," vulnerability by using ",(0,r.kt)("inlineCode",{parentName:"p"},"try_get")," for safer access and ensuring the map keys and values have compatible types throughout the process."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"    #[contractimpl]\n    impl UnsafeMapGet {\n        pub fn get_map_with_different_values(env: Env, key: i32) -> Result<Option<i32>, Error> {\n            let map: Map<Val, Val> = map![\n                &env,\n                (1i32.into_val(&env), 2i32.into_val(&env)),\n                (3i32.into_val(&env), 4i64.into_val(&env)),\n            ];\n            let map: Val = map.into();\n            let map: Map<i32, i32> = map.try_into_val(&env).unwrap();\n            map.try_get(key).map_err(Error::from)\n        }\n\n        pub fn get_map_with_different_keys(env: Env, key: i32) -> Result<Option<i32>, Error> {\n            let map: Map<Val, Val> = map![\n                &env,\n                (1i32.into_val(&env), 2i32.into_val(&env)),\n                (3i64.into_val(&env), 4i32.into_val(&env)),\n            ];\n            let map: Val = map.into();\n            let map: Map<i32, i32> = map.try_into_val(&env).unwrap();\n            map.try_get(key).map_err(Error::from)\n        }\n    }\n")),(0,r.kt)("p",null,"The remediated code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-map-get/unsafe-map-get-1/remediated-example"},"here"),"."))}m.isMDXComponent=!0}}]);