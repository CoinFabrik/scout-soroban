"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[2680],{9613:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>f});var n=r(9496);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var l=n.createContext({}),c=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},p=function(e){var t=c(e.components);return n.createElement(l.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),u=c(r),m=o,f=u["".concat(l,".").concat(m)]||u[m]||d[m]||a;return r?n.createElement(f,i(i({ref:t},p),{},{components:r})):n.createElement(f,i({ref:t},p))}));function f(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[u]="string"==typeof e?e:o,i[1]=s;for(var c=2;c<a;c++)i[c]=r[c];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},9457:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>d,frontMatter:()=>a,metadata:()=>s,toc:()=>c});var n=r(2564),o=(r(9496),r(9613));const a={},i="Assert  violation",s={unversionedId:"detectors/assert-violation",id:"detectors/assert-violation",title:"Assert  violation",description:"What it does\u200b",source:"@site/docs/detectors/15-assert-violation.md",sourceDirName:"detectors",slug:"/detectors/assert-violation",permalink:"/scout-soroban/docs/detectors/assert-violation",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/15-assert-violation.md",tags:[],version:"current",sidebarPosition:15,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Iterators-over-indexing",permalink:"/scout-soroban/docs/detectors/iterators-over-indexing"},next:{title:"Unprotected Mapping Operation",permalink:"/scout-soroban/docs/detectors/unprotected-mapping-operation"}},l={},c=[{value:"What it does\u200b",id:"what-it-does",level:3},{value:"Why is this bad?\u200b",id:"why-is-this-bad",level:3},{value:"Example\u200b",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],p={toc:c},u="wrapper";function d(e){let{components:t,...r}=e;return(0,o.kt)(u,(0,n.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"assert--violation"},"Assert  violation"),(0,o.kt)("h3",{id:"what-it-does"},"What it does\u200b"),(0,o.kt)("p",null,"Checks for ",(0,o.kt)("inlineCode",{parentName:"p"},"assert!")," macro usage."),(0,o.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?\u200b"),(0,o.kt)("p",null,"The ",(0,o.kt)("inlineCode",{parentName:"p"},"assert!")," macro can cause the contract to panic."),(0,o.kt)("h3",{id:"example"},"Example\u200b"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'pub fn assert_if_greater_than_10(_env: Env, value: u128) -> bool {\n       assert!(value <= 10, "value should be less than 10");\n       true\n   }\n\n')),(0,o.kt)("p",null,"Use instead:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"pub fn assert_if_greater_than_10(_env: Env, value: u128) -> Result<bool, AVError> {\n       if value <= 10 {\n           Ok(true)\n       } else {\n           Err(AVError::GreaterThan10)\n       }\n   }\n")),(0,o.kt)("h3",{id:"implementation"},"Implementation"),(0,o.kt)("p",null,"The detector's implementation can be found at ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/assert-violation"},"this link"),"."))}d.isMDXComponent=!0}}]);