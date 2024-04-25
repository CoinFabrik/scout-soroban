"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[672],{9613:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>m});var n=r(9496);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function s(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function i(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var c=n.createContext({}),l=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):s(s({},t),e)),r},p=function(e){var t=l(e.components);return n.createElement(c.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,c=e.parentName,p=i(e,["components","mdxType","originalType","parentName"]),u=l(r),f=o,m=u["".concat(c,".").concat(f)]||u[f]||d[f]||a;return r?n.createElement(m,s(s({ref:t},p),{},{components:r})):n.createElement(m,s({ref:t},p))}));function m(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,s=new Array(a);s[0]=f;var i={};for(var c in t)hasOwnProperty.call(t,c)&&(i[c]=t[c]);i.originalType=e,i[u]="string"==typeof e?e:o,s[1]=i;for(var l=2;l<a;l++)s[l]=r[l];return n.createElement.apply(null,s)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},3161:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>c,contentTitle:()=>s,default:()=>d,frontMatter:()=>a,metadata:()=>i,toc:()=>l});var n=r(2564),o=(r(9496),r(9613));const a={},s="Unsafe expect",i={unversionedId:"detectors/unsafe-expect",id:"detectors/unsafe-expect",title:"Unsafe expect",description:"What it does",source:"@site/docs/detectors/3-unsafe-expect.md",sourceDirName:"detectors",slug:"/detectors/unsafe-expect",permalink:"/scout-soroban/docs/detectors/unsafe-expect",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/3-unsafe-expect.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Unsafe unwrap",permalink:"/scout-soroban/docs/detectors/unsafe-unwrap"},next:{title:"Overflow-check",permalink:"/scout-soroban/docs/detectors/overflow-check"}},c={},l=[{value:"What it does",id:"what-it-does",level:3},{value:"Why is this bad?",id:"why-is-this-bad",level:3},{value:"Example",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],p={toc:l},u="wrapper";function d(e){let{components:t,...r}=e;return(0,o.kt)(u,(0,n.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"unsafe-expect"},"Unsafe expect"),(0,o.kt)("h3",{id:"what-it-does"},"What it does"),(0,o.kt)("p",null,"Checks for usage of ",(0,o.kt)("inlineCode",{parentName:"p"},".expect()")),(0,o.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?"),(0,o.kt)("p",null,(0,o.kt)("inlineCode",{parentName:"p"},".expect()")," might panic if the result value is an error or ",(0,o.kt)("inlineCode",{parentName:"p"},"None"),"."),(0,o.kt)("h3",{id:"example"},"Example"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'// example code where a warning is issued\nfn main() {\n    let result = result_fn().expect("error");\n}\nfn result_fn() -> Result<u8, Error> {\n    Err(Error::new(ErrorKind::Other, "error"))\n}\n')),(0,o.kt)("p",null,"Use instead:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'// example code that does not raise a warning\nfn main() {\n   let result = if let Ok(result) = result_fn() {\n      result\n  }\n}\nfn result_fn() -> Result<u8, Error> {\n    Err(Error::new(ErrorKind::Other, "error"))\n}\n')),(0,o.kt)("h3",{id:"implementation"},"Implementation"),(0,o.kt)("p",null,"The detector's implementation can be found at ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-expect"},"this link"),"."))}d.isMDXComponent=!0}}]);