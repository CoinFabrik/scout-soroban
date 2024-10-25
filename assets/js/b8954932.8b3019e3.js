"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[4735],{9613:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>m});var n=r(9496);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function s(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function i(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var c=n.createContext({}),l=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):s(s({},t),e)),r},u=function(e){var t=l(e.components);return n.createElement(c.Provider,{value:t},e.children)},d="mdxType",p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,o=e.originalType,c=e.parentName,u=i(e,["components","mdxType","originalType","parentName"]),d=l(r),f=a,m=d["".concat(c,".").concat(f)]||d[f]||p[f]||o;return r?n.createElement(m,s(s({ref:t},u),{},{components:r})):n.createElement(m,s({ref:t},u))}));function m(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=r.length,s=new Array(o);s[0]=f;var i={};for(var c in t)hasOwnProperty.call(t,c)&&(i[c]=t[c]);i.originalType=e,i[d]="string"==typeof e?e:a,s[1]=i;for(var l=2;l<o;l++)s[l]=r[l];return n.createElement.apply(null,s)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},3472:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>c,contentTitle:()=>s,default:()=>p,frontMatter:()=>o,metadata:()=>i,toc:()=>l});var n=r(2564),a=(r(9496),r(9613));const o={},s="Unrestricted transfer from",i={unversionedId:"detectors/unrestricted-transfer-from",id:"detectors/unrestricted-transfer-from",title:"Unrestricted transfer from",description:"Description",source:"@site/docs/detectors/18-unrestricted-transfer-from.md",sourceDirName:"detectors",slug:"/detectors/unrestricted-transfer-from",permalink:"/scout-soroban/docs/detectors/unrestricted-transfer-from",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/18-unrestricted-transfer-from.md",tags:[],version:"current",sidebarPosition:18,frontMatter:{},sidebar:"docsSidebar",previous:{title:"DoS unexpected revert with vector",permalink:"/scout-soroban/docs/detectors/dos-unexpected-revert-with-vector"},next:{title:"Unsafe map get",permalink:"/scout-soroban/docs/detectors/unsafe-map-get"}},c={},l=[{value:"Description",id:"description",level:2},{value:"Why is this bad?",id:"why-is-this-bad",level:2},{value:"Issue example",id:"issue-example",level:2},{value:"Remediated example",id:"remediated-example",level:2},{value:"How is it detected?",id:"how-is-it-detected",level:2},{value:"References",id:"references",level:2}],u={toc:l},d="wrapper";function p(e){let{components:t,...r}=e;return(0,a.kt)(d,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"unrestricted-transfer-from"},"Unrestricted transfer from"),(0,a.kt)("h2",{id:"description"},"Description"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Category: ",(0,a.kt)("inlineCode",{parentName:"li"},"Validations and error handling")),(0,a.kt)("li",{parentName:"ul"},"Severity: ",(0,a.kt)("inlineCode",{parentName:"li"},"High")),(0,a.kt)("li",{parentName:"ul"},"Detectors: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unrestricted-transfer-from"},(0,a.kt)("inlineCode",{parentName:"a"},"unrestricted-transfer-from"))),(0,a.kt)("li",{parentName:"ul"},"Test Cases: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1"},(0,a.kt)("inlineCode",{parentName:"a"},"unrestricted-transfer-from-1")))),(0,a.kt)("p",null,"Allowing unrestricted ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from")," operations poses a significant issue. When ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," arguments for that function is provided directly by the user, this might enable the withdrawal of funds from any actor with token approval on the contract. "),(0,a.kt)("h2",{id:"why-is-this-bad"},"Why is this bad?"),(0,a.kt)("p",null,"The absence of proper authorization checks for sensitive operations, like ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from"),", can lead to the loss of funds or other undesired consequences. For example, if a user, Alice, approves a contract to spend her tokens, and the contract lacks proper authorization checks, another user, Bob, could invoke the contract and potentially transfer Alice's tokens to himself without her explicit consent."),(0,a.kt)("h2",{id:"issue-example"},"Issue example"),(0,a.kt)("p",null,"Consider the following ",(0,a.kt)("inlineCode",{parentName:"p"},"Soroban")," function:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"     pub fn deposit(env: Env, from: Address) -> Result<(), UTFError> {\n        let mut state: State = Self::get_state(env.clone())?;\n        state.buyer.require_auth();\n        if state.status != Status::Created {\n            return Err(UTFError::StatusMustBeCreated);\n        }\n        let token_client = token::Client::new(&env, &state.token);\n        token_client.transfer_from(\n            &env.current_contract_address(),\n            &from,\n            &env.current_contract_address(),\n            &state.amount,\n        );\n        state.status = Status::Locked;\n        env.storage().instance().set(&STATE, &state);\n        Ok(())\n    }\n")),(0,a.kt)("p",null,"The issue in this ",(0,a.kt)("inlineCode",{parentName:"p"},"deposit")," function arises from the use of ",(0,a.kt)("inlineCode",{parentName:"p"},"from"),", an user-defined parameter as an argument in the ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," field of the ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from")," function. Alice can approve a contract to spend their tokens, then Bob can call that contract, use that allowance to send as themselves Alice's tokens."),(0,a.kt)("p",null,"The code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/vulnerable-example"},(0,a.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,a.kt)("h2",{id:"remediated-example"},"Remediated example"),(0,a.kt)("p",null,"Avoid using user-defined arguments as ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," parameter in ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from"),". Instead, use ",(0,a.kt)("inlineCode",{parentName:"p"},"state.buyer")," as shown in the following example."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"     pub fn deposit(env: Env) -> Result<(), UTFError> {\n        let mut state: State = Self::get_state(env.clone())?;\n        state.buyer.require_auth();\n        if state.status != Status::Created {\n            return Err(UTFError::StatusMustBeCreated);\n        }\n        let token_client = token::Client::new(&env, &state.token);\n        token_client.transfer_from(\n            &env.current_contract_address(),\n            &state.buyer,\n            &env.current_contract_address(),\n            &state.amount,\n        );\n        state.status = Status::Locked;\n        env.storage().instance().set(&STATE, &state);\n        Ok(())\n    }\n")),(0,a.kt)("p",null,"The remediated code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/remediated-example"},"here"),"."),(0,a.kt)("h2",{id:"how-is-it-detected"},"How is it detected?"),(0,a.kt)("p",null,"It warns you if a ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from")," function is called with a user-defined parameter in the ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," field."),(0,a.kt)("h2",{id:"references"},"References"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#arbitrary-from-in-transferfrom"},"Slither: Arbitrary from in transferFrom"))))}p.isMDXComponent=!0}}]);