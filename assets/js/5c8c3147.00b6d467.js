"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[5038],{9613:(e,t,a)=>{a.d(t,{Zo:()=>p,kt:()=>h});var n=a(9496);function r(e,t,a){return t in e?Object.defineProperty(e,t,{value:a,enumerable:!0,configurable:!0,writable:!0}):e[t]=a,e}function o(e,t){var a=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),a.push.apply(a,n)}return a}function i(e){for(var t=1;t<arguments.length;t++){var a=null!=arguments[t]?arguments[t]:{};t%2?o(Object(a),!0).forEach((function(t){r(e,t,a[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(a)):o(Object(a)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(a,t))}))}return e}function l(e,t){if(null==e)return{};var a,n,r=function(e,t){if(null==e)return{};var a,n,r={},o=Object.keys(e);for(n=0;n<o.length;n++)a=o[n],t.indexOf(a)>=0||(r[a]=e[a]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)a=o[n],t.indexOf(a)>=0||Object.prototype.propertyIsEnumerable.call(e,a)&&(r[a]=e[a])}return r}var s=n.createContext({}),d=function(e){var t=n.useContext(s),a=t;return e&&(a="function"==typeof e?e(t):i(i({},t),e)),a},p=function(e){var t=d(e.components);return n.createElement(s.Provider,{value:t},e.children)},u="mdxType",c={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var a=e.components,r=e.mdxType,o=e.originalType,s=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),u=d(a),m=r,h=u["".concat(s,".").concat(m)]||u[m]||c[m]||o;return a?n.createElement(h,i(i({ref:t},p),{},{components:a})):n.createElement(h,i({ref:t},p))}));function h(e,t){var a=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=a.length,i=new Array(o);i[0]=m;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[u]="string"==typeof e?e:r,i[1]=l;for(var d=2;d<o;d++)i[d]=a[d];return n.createElement.apply(null,i)}return n.createElement.apply(null,a)}m.displayName="MDXCreateElement"},2072:(e,t,a)=>{a.r(t),a.d(t,{assets:()=>s,contentTitle:()=>i,default:()=>c,frontMatter:()=>o,metadata:()=>l,toc:()=>d});var n=a(2564),r=(a(9496),a(9613));const o={},i="Scout Bug Fighter for Soroban: Improving Tool's Precision",l={unversionedId:"precision-and-recall/first-iteration",id:"precision-and-recall/first-iteration",title:"Scout Bug Fighter for Soroban: Improving Tool's Precision",description:"In the scope of the second grant awarded to CoinFabrik by the Stellar Community Fund to advance the development of Scout for Soroban, the focus extends beyond incorporating new detectors and refining features. A key objective of this grant is to subject the tool to rigorous testing against real Soroban projects. Through this process, the aim is to analyze the outcomes meticulously, identifying areas for enhancement to increase the tool's precision. This includes minimizing false positives and false negatives, thereby fortifying its efficacy.",source:"@site/docs/precision-and-recall/first-iteration.md",sourceDirName:"precision-and-recall",slug:"/precision-and-recall/first-iteration",permalink:"/scout-soroban/docs/precision-and-recall/first-iteration",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/precision-and-recall/first-iteration.md",tags:[],version:"current",frontMatter:{},sidebar:"docsSidebar",previous:{title:"Architecture",permalink:"/scout-soroban/docs/architecture"}},s={},d=[{value:"Scout for Soroban: Current Status",id:"scout-for-soroban-current-status",level:2},{value:"Validating Scout on Real Life Projects",id:"validating-scout-on-real-life-projects",level:2},{value:"Improvements on Detectors",id:"improvements-on-detectors",level:2},{value:"On <code>unsafe-unwrap</code>",id:"on-unsafe-unwrap",level:3},{value:"On <code>set-contract-storage</code>",id:"on-set-contract-storage",level:3},{value:"Enhanced Authentication Detection: Context-Aware Analysis",id:"enhanced-authentication-detection-context-aware-analysis",level:3},{value:"Improvements on Troubleshooting Documentation",id:"improvements-on-troubleshooting-documentation",level:2},{value:"Appendices",id:"appendices",level:2},{value:"Appendix 1: False Positive Alarms per Project",id:"appendix-1-false-positive-alarms-per-project",level:3},{value:"Table 1: False positives per project",id:"table-1-false-positives-per-project",level:4},{value:"Appendix 2: False Positive Alarms per Detector",id:"appendix-2-false-positive-alarms-per-detector",level:3},{value:"Table 2: False positives per detector",id:"table-2-false-positives-per-detector",level:4}],p={toc:d},u="wrapper";function c(e){let{components:t,...a}=e;return(0,r.kt)(u,(0,n.Z)({},p,a,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"scout-bug-fighter-for-soroban-improving-tools-precision"},"Scout Bug Fighter for Soroban: Improving Tool's Precision"),(0,r.kt)("p",null,"In the scope of the second grant awarded to CoinFabrik by the ",(0,r.kt)("a",{parentName:"p",href:"https://communityfund.stellar.org/"},"Stellar Community Fund")," to advance the development of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban"},"Scout for Soroban"),", the focus extends beyond incorporating new detectors and refining features. A key objective of this grant is to subject the tool to rigorous testing against real Soroban projects. Through this process, the aim is to analyze the outcomes meticulously, identifying areas for enhancement to increase the tool's precision. This includes minimizing false positives and false negatives, thereby fortifying its efficacy."),(0,r.kt)("p",null,"In this document we describe the work and achievements made during this first iteration."),(0,r.kt)("h2",{id:"scout-for-soroban-current-status"},"Scout for Soroban: Current Status"),(0,r.kt)("p",null,"At the end of January, we launched the first prototype of Scout for Soroban. Over the last two months, our focus has been on maturing the tool, taking it one step forward to make it a useful tool for every Soroban developer."),(0,r.kt)("p",null,"Currently, the tool offers the following features:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"A CLI tool."),(0,r.kt)("li",{parentName:"ul"},"Detection capabilities for 19 warnings (and growing).",(0,r.kt)("ul",{parentName:"li"},(0,r.kt)("li",{parentName:"ul"},"4 enhancement suggestions."),(0,r.kt)("li",{parentName:"ul"},"2 minor vulnerabilities."),(0,r.kt)("li",{parentName:"ul"},"7 medium vulnerabilities."),(0,r.kt)("li",{parentName:"ul"},"6 critical vulnerabilities."))),(0,r.kt)("li",{parentName:"ul"},"Different output options so that users can chose the one that best suit their needs (HTML, markdown, pdf and JSON)."),(0,r.kt)("li",{parentName:"ul"},"A ",(0,r.kt)("a",{parentName:"li",href:"https://marketplace.visualstudio.com/items?itemName=CoinFabrik.scout-audit"},"VS Code extension")," to integrate Scout into the development workspace."),(0,r.kt)("li",{parentName:"ul"},"A ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-actions"},"GitHub action")," to include Scout in the CI/CD workflow.")),(0,r.kt)("h2",{id:"validating-scout-on-real-life-projects"},"Validating Scout on Real Life Projects"),(0,r.kt)("p",null,"In order to understand how Scout fares in a real-world scenario, we ran the tool on 71 smart contracts of 18 public Soroban projects and measured its precision and recall. Precision is directly related to false positives, as the ratio of true positives over the false and true positives (or the rate of correctly triggered alarms). Recall relates to false negatives. It is the ratio of true positives over the sum of true positives and false negatives (or the rate of issues found among those available)."),(0,r.kt)("p",null,"After running Scout (",(0,r.kt)("a",{parentName:"p",href:"https://crates.io/crates/cargo-scout-audit/0.2.4"},"cargo-scout-audit version 0.2.4"),") on the smart contracts, we identified a total of 847 triggered alarms, out of which 290 were determined to be false positives following a manual review of each finding. This results in a false positive rate of 34.24%. We further analyzed the false positives associated with each detector, focusing particularly on ",(0,r.kt)("inlineCode",{parentName:"p"},"unsafe-unwrap")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"set-contract-storage"),", the two detectors with the highest number of false positives, to identify potential improvements to the tool's precision."),(0,r.kt)("p",null,"We subsequently refined the detectors and released an updated version of Scout (",(0,r.kt)("a",{parentName:"p",href:"https://crates.io/crates/cargo-scout-audit/0.2.6"},"cargo-scout-audit version 0.2.6"),"), which included enhancements. We then re-ran the tool, focusing on the revised detectors. Our modifications were not limited to the two detectors that produced false positives; we also adjusted other detectors that we believed could potentially lead to false positives in similar situations. As a result, our analysis led to improvements on five detectors."),(0,r.kt)("p",null,"In addition to analyzing Scout as a single source of triggers, we conducted two other analyses (refer to Appendices section below). Firstly, we examined the rates of false positives per smart contract/project, which reflects the perceived quality from the user's perspective (those who would run the tool in their project individually). Secondly, we assessed the rate of false positives per detector to determine the performance of each detector and identify areas needing improvement."),(0,r.kt)("p",null,"We have already begun the next iteration of Precision and Recall, focusing on further refining Scout's detectors. We will conduct new runs of the tool and analyze the results, including the latest detectors additions. This analysis will enable us to confirm the final rate of false positives after the improvements, completing Table 2: False Positives per Detector."),(0,r.kt)("h2",{id:"improvements-on-detectors"},"Improvements on Detectors"),(0,r.kt)("p",null,"As we analyzed the results from running the tool, we identified that most of the false positives occur in ",(0,r.kt)("inlineCode",{parentName:"p"},"unsafe-unwrap"),"  and ",(0,r.kt)("inlineCode",{parentName:"p"},"set-contract-storage")," detectors. We focused our work on improving the precision of these two detectors, as well as other detectors that could be enhanced from similar checks."),(0,r.kt)("h3",{id:"on-unsafe-unwrap"},"On ",(0,r.kt)("inlineCode",{parentName:"h3"},"unsafe-unwrap")),(0,r.kt)("p",null,"For ",(0,r.kt)("inlineCode",{parentName:"p"},"unsafe-unwrap"),", we noticed cases where previous checks in the analyzed code made the particular use of ",(0,r.kt)("inlineCode",{parentName:"p"},"unwrap()")," not result in an error. We updated the detector to validate whether these checks are present in the code, decreasing the amount of false positive detections on a second run of the tool."),(0,r.kt)("p",null,"Example 1: False positive for unsafe unwrap with previous check."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'pub  fn  truncating_mul(self:  &Self,  x:  i128)  ->  i128 {\n    let  result  =  safe_mul(x,  self.num,  self.den);\n    if  result.is_err(){\n    panic!("integer overflow")\n    }\n    result.unwrap()\n}\n')),(0,r.kt)("p",null,"Previously, our ",(0,r.kt)("inlineCode",{parentName:"p"},"unsafe-unwrap")," detector would generate a warning for the usage of ",(0,r.kt)("inlineCode",{parentName:"p"},"unwrap")," in this context, even though the function would never reach that part of the code without a confirmed existing value. Now, our detector can handle various scenarios for both ",(0,r.kt)("inlineCode",{parentName:"p"},"Result")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"Option")," types. It can appropriately manage cases where the user might halt execution of a function upon encountering an error (as shown above) or use unwrap safely within an ",(0,r.kt)("inlineCode",{parentName:"p"},"is_some()")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"is_ok()")," block. Furthermore, the detector is capable of addressing conditions involving or operators with types that are either the same or different, treating each case individually (e.g. using ",(0,r.kt)("inlineCode",{parentName:"p"},"is_some()")," on one variable, and ",(0,r.kt)("inlineCode",{parentName:"p"},"is_err()")," on another one)."),(0,r.kt)("p",null,"We also registered another class of false positives, which, due to the particular arithmetic and value assignment of the variables involved, would probably not result in a vulnerability, but found no way to discard them for this detector within the restrictions of our static analysis method."),(0,r.kt)("p",null,"Example 2: False positive due to arithmetic and value assignment in range. If the values assigned to variables do not exceed the range, unwrap() will not return an error."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"pub  fn  some_function(e:  &Env)  {\n    let  mut  total  =  get_total(e);\n    total  =  total.checked_add(1).unwrap();\n    total\n}\n")),(0,r.kt)("p",null,"Finally, we identified that the same checks could be applied to the detector ",(0,r.kt)("inlineCode",{parentName:"p"},"unsafe-expect"),", and updated it accordingly."),(0,r.kt)("h3",{id:"on-set-contract-storage"},"On ",(0,r.kt)("inlineCode",{parentName:"h3"},"set-contract-storage")),(0,r.kt)("p",null,"Upon analyzing false positives in the ",(0,r.kt)("inlineCode",{parentName:"p"},"set-contract-storage")," detector, we identified use cases where the authorization to use ",(0,r.kt)("inlineCode",{parentName:"p"},"env.storage()")," was done in a function outside of the analysis context of our detector, or the storage method being detected (e.g: get) did not represent a vulnerability.  "),(0,r.kt)("p",null,"We extended the analysis context of our detector to identify these authorizations in parent functions and added the capability for the detector to now differentiate between various storage types from the Soroban SDK."),(0,r.kt)("p",null,"Example 3: False positive for set contract storage. This example authorizes the call of storage and uses get which is non vulnerable."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"pub  fn  some_function(  env:  Env)  ->  u32  {\n    let  storage  =  env.storage().persistent();\n    if  storage\n        .get::<_,  SomeState>(&DataKey::SomeState)\n        .is_none()  {\n        panic_with_error!(&env,  Error::NotInitialized);\n    }\n\n    let  admin  =  storage.get::<_,  Address (&DataKey::Admin).unwrap();\n    admin.require_auth();\n        ...\n}\n")),(0,r.kt)("p",null,"On the other hand, we believe that some use cases using DataKey could now result in true positives, which are being discarded after the detector\u2019s update to differentiate between storage types. When the key used to modify the storage is not of type ",(0,r.kt)("inlineCode",{parentName:"p"},"soroban_sdk:Address"),", but an enum ",(0,r.kt)("inlineCode",{parentName:"p"},"DataKey"),", the detector overlooks the issue, without validating if a user address is being modified and if it represents a vulnerability. We are currently evaluating these cases to amend our detector."),(0,r.kt)("p",null,"Example 4: New False negative for set contract storage. This example is no longer detected after our update because the ",(0,r.kt)("inlineCode",{parentName:"p"},"DataKey")," is not of type ",(0,r.kt)("inlineCode",{parentName:"p"},"soroban_sdk:Address"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"env.storage().instance().set(&DataKey::State,  &State::new(storage,  adder,  subber));\n")),(0,r.kt)("p",null,"The same extension of the analysis context was also applied on detectors ",(0,r.kt)("inlineCode",{parentName:"p"},"unprotected-mapping-operation")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"unprotected-update-contract-wasm"),"."),(0,r.kt)("h3",{id:"enhanced-authentication-detection-context-aware-analysis"},"Enhanced Authentication Detection: Context-Aware Analysis"),(0,r.kt)("p",null,"We have introduced a new feature that significantly enhances the capability of many of our detectors by making them inter-procedural context-aware. Previously, many authentication patterns caused our detectors to issue false warnings: alerts that were triggered even when the necessary verifications had been correctly executed. Our refined approach involves creating a map that includes methods and the methods they invoke. This allows us to defer analysis until all relevant methods have been reviewed. By doing this, we can maintain a graph of functions, aimed at minimizing false positives. This enhancement is particularly beneficial for authentication-related detectors, as it enables the construction of a tree of authenticated methods, ensuring more accurate detection and fewer errors."),(0,r.kt)("h2",{id:"improvements-on-troubleshooting-documentation"},"Improvements on Troubleshooting Documentation"),(0,r.kt)("p",null,"As we used Scout over a variety of projects, we noticed some issues when running the tool on contracts performing crossed calls. For these cases we found that a solution is compiling the second contract first (",(0,r.kt)("inlineCode",{parentName:"p"},"soroban contract build"),") before running Scout on the first one."),(0,r.kt)("p",null,"On the other hand, as we tried Scout on different environments, we noticed some installation caveats. We wrote down a ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/blob/main/docs/docs/intro.md#troubleshooting-guide"},"troubleshooting guide")," to aid the user on particular installation issues."),(0,r.kt)("h2",{id:"appendices"},"Appendices"),(0,r.kt)("h3",{id:"appendix-1-false-positive-alarms-per-project"},"Appendix 1: False Positive Alarms per Project"),(0,r.kt)("p",null,"Analyzing the number of false positives per project, we observe an average rate of false positives vs total positives of 21 %, and a median of 0%."),(0,r.kt)("p",null,"If we analyze only projects with detections, the average rate of false positives vs total positives per project increases to 48%, and the median to 51%."),(0,r.kt)("p",null,"We keep the identity of the analyzed projects anonymous as we confirm and responsibly disclose true positives found during our analysis of the tool\u2019s output."),(0,r.kt)("p",null,"The ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/238b728b89ab9f549a6fd189d4ac9c90bac4977c?tab=readme-ov-file#detectors"},"detectors run correspond to the ones available in the Scout version 0.2.4 at the commencement of this analysis"),"."),(0,r.kt)("h4",{id:"table-1-false-positives-per-project"},"Table 1: False positives per project"),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:null},"Project ID"),(0,r.kt)("th",{parentName:"tr",align:null},"Total Positives"),(0,r.kt)("th",{parentName:"tr",align:null},"False Positives"),(0,r.kt)("th",{parentName:"tr",align:null},"% False Positives"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"1"),(0,r.kt)("td",{parentName:"tr",align:null},"27"),(0,r.kt)("td",{parentName:"tr",align:null},"4"),(0,r.kt)("td",{parentName:"tr",align:null},"15%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"2"),(0,r.kt)("td",{parentName:"tr",align:null},"20"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"3"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"4"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"5"),(0,r.kt)("td",{parentName:"tr",align:null},"50"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"6"),(0,r.kt)("td",{parentName:"tr",align:null},"55"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"7"),(0,r.kt)("td",{parentName:"tr",align:null},"10"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"8"),(0,r.kt)("td",{parentName:"tr",align:null},"48"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"9"),(0,r.kt)("td",{parentName:"tr",align:null},"72"),(0,r.kt)("td",{parentName:"tr",align:null},"35"),(0,r.kt)("td",{parentName:"tr",align:null},"49%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"10"),(0,r.kt)("td",{parentName:"tr",align:null},"25"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"11"),(0,r.kt)("td",{parentName:"tr",align:null},"122"),(0,r.kt)("td",{parentName:"tr",align:null},"17"),(0,r.kt)("td",{parentName:"tr",align:null},"14%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"12"),(0,r.kt)("td",{parentName:"tr",align:null},"44"),(0,r.kt)("td",{parentName:"tr",align:null},"17"),(0,r.kt)("td",{parentName:"tr",align:null},"39%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"13"),(0,r.kt)("td",{parentName:"tr",align:null},"10"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"14"),(0,r.kt)("td",{parentName:"tr",align:null},"70"),(0,r.kt)("td",{parentName:"tr",align:null},"63"),(0,r.kt)("td",{parentName:"tr",align:null},"90%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"15"),(0,r.kt)("td",{parentName:"tr",align:null},"12"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"16"),(0,r.kt)("td",{parentName:"tr",align:null},"15"),(0,r.kt)("td",{parentName:"tr",align:null},"10"),(0,r.kt)("td",{parentName:"tr",align:null},"67%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"17"),(0,r.kt)("td",{parentName:"tr",align:null},"47"),(0,r.kt)("td",{parentName:"tr",align:null},"28"),(0,r.kt)("td",{parentName:"tr",align:null},"60%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"18"),(0,r.kt)("td",{parentName:"tr",align:null},"220"),(0,r.kt)("td",{parentName:"tr",align:null},"116"),(0,r.kt)("td",{parentName:"tr",align:null},"53%")))),(0,r.kt)("h3",{id:"appendix-2-false-positive-alarms-per-detector"},"Appendix 2: False Positive Alarms per Detector"),(0,r.kt)("p",null,"In the following table, we identify the total number of positives and false positives per detector across all analyzed smart contracts. The ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/238b728b89ab9f549a6fd189d4ac9c90bac4977c?tab=readme-ov-file#detectors"},"detectors run correspond to the ones available in the Scout version 0.2.4 at the beginning of this analysis"),". Notice that some detectors were never activated in the analyzed code. The false positives were analyzed in order to improve the detectors."),(0,r.kt)("h4",{id:"table-2-false-positives-per-detector"},"Table 2: False positives per detector"),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"th"},"Detector")),(0,r.kt)("th",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"th"},"Total Positives")),(0,r.kt)("th",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"th"},"False Positives")),(0,r.kt)("th",{parentName:"tr",align:"right"},(0,r.kt)("strong",{parentName:"th"},"% False Positives")))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Divide before multiply"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("em",{parentName:"td"},"Unsafe unwrap")),(0,r.kt)("td",{parentName:"tr",align:null},"180"),(0,r.kt)("td",{parentName:"tr",align:null},"6"),(0,r.kt)("td",{parentName:"tr",align:"right"},"3.33%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("em",{parentName:"td"},"Unsafe expect")),(0,r.kt)("td",{parentName:"tr",align:null},"65"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Overflow check"),(0,r.kt)("td",{parentName:"tr",align:null},"2"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Insufficiently random values"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("em",{parentName:"td"},"Unprotected update current contract wasm")),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Avoid core mem forget"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("em",{parentName:"td"},"Set contract storage")),(0,r.kt)("td",{parentName:"tr",align:null},"478"),(0,r.kt)("td",{parentName:"tr",align:null},"284"),(0,r.kt)("td",{parentName:"tr",align:"right"},"59.41%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Avoid panic error"),(0,r.kt)("td",{parentName:"tr",align:null},"63"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Avoid unsafe block"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Dos unbounded operation"),(0,r.kt)("td",{parentName:"tr",align:null},"13"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Soroban version"),(0,r.kt)("td",{parentName:"tr",align:null},"46"),(0,r.kt)("td",{parentName:"tr",align:null},"0"),(0,r.kt)("td",{parentName:"tr",align:"right"},"0.00%")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Total")),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"847")),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"290")),(0,r.kt)("td",{parentName:"tr",align:"right"},(0,r.kt)("strong",{parentName:"td"},"34.24%"))))),(0,r.kt)("p",null,"Some of the detectors named above are ",(0,r.kt)("em",{parentName:"p"},"highlighted"),", meaning we focused our analysis on them and worked primarily to improve those."))}c.isMDXComponent=!0}}]);