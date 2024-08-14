"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[8815],{9613:(e,t,n)=>{n.d(t,{Zo:()=>d,kt:()=>h});var r=n(9496);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function c(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var s=r.createContext({}),u=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):c(c({},t),e)),n},d=function(e){var t=u(e.components);return r.createElement(s.Provider,{value:t},e.children)},p="mdxType",l={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,s=e.parentName,d=i(e,["components","mdxType","originalType","parentName"]),p=u(n),m=a,h=p["".concat(s,".").concat(m)]||p[m]||l[m]||o;return n?r.createElement(h,c(c({ref:t},d),{},{components:n})):r.createElement(h,c({ref:t},d))}));function h(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,c=new Array(o);c[0]=m;var i={};for(var s in t)hasOwnProperty.call(t,s)&&(i[s]=t[s]);i.originalType=e,i[p]="string"==typeof e?e:a,c[1]=i;for(var u=2;u<o;u++)c[u]=n[u];return r.createElement.apply(null,c)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},1440:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>c,default:()=>l,frontMatter:()=>o,metadata:()=>i,toc:()=>u});var r=n(2564),a=(n(9496),n(9613));const o={},c="Unprotected update current contract wasm",i={unversionedId:"detectors/unprotected-update-current-contract-wasm",id:"detectors/unprotected-update-current-contract-wasm",title:"Unprotected update current contract wasm",description:"Description",source:"@site/docs/detectors/6-unprotected-update-current-contract-wasm.md",sourceDirName:"detectors",slug:"/detectors/unprotected-update-current-contract-wasm",permalink:"/scout-soroban/docs/detectors/unprotected-update-current-contract-wasm",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/6-unprotected-update-current-contract-wasm.md",tags:[],version:"current",sidebarPosition:6,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Insufficiently random values",permalink:"/scout-soroban/docs/detectors/insufficiently-random-values"},next:{title:"Avoid core::mem::forget usage",permalink:"/scout-soroban/docs/detectors/avoid-core-mem-forget"}},s={},u=[{value:"Description",id:"description",level:2},{value:"Why is this bad?",id:"why-is-this-bad",level:2},{value:"Issue example",id:"issue-example",level:2},{value:"Remediated example",id:"remediated-example",level:2},{value:"How is it detected?",id:"how-is-it-detected",level:2}],d={toc:u},p="wrapper";function l(e){let{components:t,...n}=e;return(0,a.kt)(p,(0,r.Z)({},d,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"unprotected-update-current-contract-wasm"},"Unprotected update current contract wasm"),(0,a.kt)("h2",{id:"description"},"Description"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Category: ",(0,a.kt)("inlineCode",{parentName:"li"},"Authorization")),(0,a.kt)("li",{parentName:"ul"},"Severity: ",(0,a.kt)("inlineCode",{parentName:"li"},"Critical")),(0,a.kt)("li",{parentName:"ul"},"Detector: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unprotected-update-current-contract-wasm"},(0,a.kt)("inlineCode",{parentName:"a"},"unprotected-update-current-contract-wasm"))),(0,a.kt)("li",{parentName:"ul"},"Test Cases: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-update-current-contract-wasm/unprotected-update-current-contract-wasm-1"},(0,a.kt)("inlineCode",{parentName:"a"},"unprotected-update-current-contract-wasm-1"))," ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-update-current-contract-wasm/unprotected-update-current-contract-wasm-2"},(0,a.kt)("inlineCode",{parentName:"a"},"unprotected-update-current-contract-wasm-2"))," ")),(0,a.kt)("p",null,"It warns you if ",(0,a.kt)("inlineCode",{parentName:"p"},"update_current_contract_wasm()")," function is called without a previous check of the address of the caller. "),(0,a.kt)("h2",{id:"why-is-this-bad"},"Why is this bad?"),(0,a.kt)("p",null,"If users are allowed to call ",(0,a.kt)("inlineCode",{parentName:"p"},"update_current_contract_wasm()"),", they can intentionally modify the contract behaviour, leading to the loss of all associated data/tokens and functionalities given by this contract or by others that depend on it."),(0,a.kt)("h2",{id:"issue-example"},"Issue example"),(0,a.kt)("p",null,"Consider the following ",(0,a.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"\n #[contractimpl]\nimpl UpgradeableContract {\n    pub fn init(e: Env, admin: Address) {\n        e.storage().instance().set(&DataKey::Admin, &admin);\n    }\n\n    pub fn version() -> u32 {\n        1\n    }\n\n    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {\n        e.deployer().update_current_contract_wasm(new_wasm_hash);\n    }\n}\n\n")),(0,a.kt)("p",null,"This contract allows upgrades through the ",(0,a.kt)("inlineCode",{parentName:"p"},"update_current_contract_wasm")," function. If just anyone can call this function, they could modify the contract behaviour."),(0,a.kt)("p",null,"The code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-update-current-contract-wasm/unprotected-update-current-contract-wasm-1/vulnerable-example"},"here"),"."),(0,a.kt)("h2",{id:"remediated-example"},"Remediated example"),(0,a.kt)("p",null,"To prevent this, the function should be restricted to administrators or authorized users only."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"\n #[contractimpl]\nimpl UpgradeableContract {\n    pub fn init(e: Env, admin: Address) {\n        e.storage().instance().set(&DataKey::Admin, &admin);\n    }\n\n    pub fn version() -> u32 {\n        1\n    }\n\n    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {\n        let admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();\n        admin.require_auth();\n\n        e.deployer().update_current_contract_wasm(new_wasm_hash);\n    }\n}\n")),(0,a.kt)("p",null,"The remediated code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-update-current-contract-wasm/unprotected-update-current-contract-wasm-1/remediated-example"},"here"),"."),(0,a.kt)("h2",{id:"how-is-it-detected"},"How is it detected?"),(0,a.kt)("p",null,"It warns you if ",(0,a.kt)("inlineCode",{parentName:"p"},"update_current_contract_wasm()")," function is called without a previous check of the address of the caller."))}l.isMDXComponent=!0}}]);