"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[936],{9613:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>m});var n=r(9496);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var l=n.createContext({}),c=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},u=function(e){var t=c(e.components);return n.createElement(l.Provider,{value:t},e.children)},p="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,l=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),p=c(r),f=a,m=p["".concat(l,".").concat(f)]||p[f]||d[f]||i;return r?n.createElement(m,o(o({ref:t},u),{},{components:r})):n.createElement(m,o({ref:t},u))}));function m(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,o=new Array(i);o[0]=f;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[p]="string"==typeof e?e:a,o[1]=s;for(var c=2;c<i;c++)o[c]=r[c];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},5242:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>o,default:()=>d,frontMatter:()=>i,metadata:()=>s,toc:()=>c});var n=r(2564),a=(r(9496),r(9613));const i={},o="Unrestricted Transfer From",s={unversionedId:"vulnerabilities/unrestricted-transfer-from",id:"vulnerabilities/unrestricted-transfer-from",title:"Unrestricted Transfer From",description:"Description",source:"@site/docs/vulnerabilities/18-unrestricted-transfer-from.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/unrestricted-transfer-from",permalink:"/scout-soroban/docs/vulnerabilities/unrestricted-transfer-from",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/vulnerabilities/18-unrestricted-transfer-from.md",tags:[],version:"current",sidebarPosition:18,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Unprotected mapping operation",permalink:"/scout-soroban/docs/vulnerabilities/unprotected-mapping-operation"},next:{title:"Unsafe map get",permalink:"/scout-soroban/docs/vulnerabilities/unsafe-map-get"}},l={},c=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Remediation",id:"remediation",level:2},{value:"References",id:"references",level:2}],u={toc:c},p="wrapper";function d(e){let{components:t,...r}=e;return(0,a.kt)(p,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"unrestricted-transfer-from"},"Unrestricted Transfer From"),(0,a.kt)("h2",{id:"description"},"Description"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,a.kt)("inlineCode",{parentName:"li"},"Validations and error handling")),(0,a.kt)("li",{parentName:"ul"},"Vulnerability Severity: ",(0,a.kt)("inlineCode",{parentName:"li"},"High")),(0,a.kt)("li",{parentName:"ul"},"Detectors: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unrestricted-transfer-from"},(0,a.kt)("inlineCode",{parentName:"a"},"unrestricted-transfer-from"))),(0,a.kt)("li",{parentName:"ul"},"Test Cases: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1"},(0,a.kt)("inlineCode",{parentName:"a"},"unrestricted-transfer-from-1")))),(0,a.kt)("p",null,"Allowing unrestricted ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from")," operations poses a significant vulnerability. When ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," arguments for that function is provided directly by the user, this might enable the withdrawal of funds from any actor with token approval on the contract. This could result in unauthorized transfers and loss of funds. "),(0,a.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,a.kt)("p",null,"Consider the following ",(0,a.kt)("inlineCode",{parentName:"p"},"Soroban")," function:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"     pub fn deposit(env: Env, from: Address) -> Result<(), UTFError> {\n        let mut state: State = Self::get_state(env.clone())?;\n        state.buyer.require_auth();\n        if state.status != Status::Created {\n            return Err(UTFError::StatusMustBeCreated);\n        }\n        let token_client = token::Client::new(&env, &state.token);\n        token_client.transfer_from(\n            &env.current_contract_address(),\n            &from,\n            &env.current_contract_address(),\n            &state.amount,\n        );\n        state.status = Status::Locked;\n        env.storage().instance().set(&STATE, &state);\n        Ok(())\n    }\n")),(0,a.kt)("p",null,"The vulnerability in this ",(0,a.kt)("inlineCode",{parentName:"p"},"deposit")," function arises from the use of ",(0,a.kt)("inlineCode",{parentName:"p"},"from"),", an user-defined parameter as an argument in the ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," field of the ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from")," function. Alice can approve a contract to spend their tokens, then Bob can call that contract, use that allowance to send as themselves Alice's tokens."),(0,a.kt)("p",null,"The vulnerable code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/vulnerable-example"},(0,a.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,a.kt)("h2",{id:"remediation"},"Remediation"),(0,a.kt)("p",null,"Avoid using user-defined arguments as ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," parameter in ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from"),". Instead, use ",(0,a.kt)("inlineCode",{parentName:"p"},"state.buyer")," as shown in the following example."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"     pub fn deposit(env: Env) -> Result<(), UTFError> {\n        let mut state: State = Self::get_state(env.clone())?;\n        state.buyer.require_auth();\n        if state.status != Status::Created {\n            return Err(UTFError::StatusMustBeCreated);\n        }\n        let token_client = token::Client::new(&env, &state.token);\n        token_client.transfer_from(\n            &env.current_contract_address(),\n            &state.buyer,\n            &env.current_contract_address(),\n            &state.amount,\n        );\n        state.status = Status::Locked;\n        env.storage().instance().set(&STATE, &state);\n        Ok(())\n    }\n")),(0,a.kt)("p",null,"The remediated code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/remediated-example"},(0,a.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,a.kt)("h2",{id:"references"},"References"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#arbitrary-from-in-transferfrom"},"Slither: Arbitrary from in transferFrom"))))}d.isMDXComponent=!0}}]);