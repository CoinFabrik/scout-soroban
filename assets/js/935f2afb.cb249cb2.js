"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[53],{1109:e=>{e.exports=JSON.parse('{"pluginId":"default","version":"current","label":"Next","banner":null,"badge":false,"noIndex":false,"className":"docs-version-current","isLast":true,"docsSidebars":{"docsSidebar":[{"type":"link","label":"Getting Started","href":"/scout-soroban/docs/intro","docId":"intro"},{"type":"category","label":"Vulnerabilities","collapsible":true,"collapsed":true,"items":[{"type":"link","label":"Divide before multiply","href":"/scout-soroban/docs/vulnerabilities/divide-before-multiply","docId":"vulnerabilities/divide-before-multiply"},{"type":"link","label":"Unsafe unwrap","href":"/scout-soroban/docs/vulnerabilities/unsafe-unwrap","docId":"vulnerabilities/unsafe-unwrap"},{"type":"link","label":"Unsafe expect","href":"/scout-soroban/docs/vulnerabilities/unsafe-expect","docId":"vulnerabilities/unsafe-expect"},{"type":"link","label":"Overflow check","href":"/scout-soroban/docs/vulnerabilities/overflow-check","docId":"vulnerabilities/overflow-check"},{"type":"link","label":"Insufficiently random values","href":"/scout-soroban/docs/vulnerabilities/insufficiently-random-values","docId":"vulnerabilities/insufficiently-random-values"},{"type":"link","label":"Unprotected update current contract wasm","href":"/scout-soroban/docs/vulnerabilities/unprotected-update-current-contract-wasm","docId":"vulnerabilities/unprotected-update-current-contract-wasm"},{"type":"link","label":"Avoid core::mem::forget usage","href":"/scout-soroban/docs/vulnerabilities/avoid-core-mem-forget","docId":"vulnerabilities/avoid-core-mem-forget"},{"type":"link","label":"Set contract storage","href":"/scout-soroban/docs/vulnerabilities/set-contract-storage","docId":"vulnerabilities/set-contract-storage"},{"type":"link","label":"Avoid unsafe block","href":"/scout-soroban/docs/vulnerabilities/avoid-unsafe-block","docId":"vulnerabilities/avoid-unsafe-block"},{"type":"link","label":"Soroban version","href":"/scout-soroban/docs/vulnerabilities/soroban-version","docId":"vulnerabilities/soroban-version"},{"type":"link","label":"Iterators over indexing","href":"/scout-soroban/docs/vulnerabilities/iterators-over-indexing","docId":"vulnerabilities/iterators-over-indexing"},{"type":"link","label":"Assert violation","href":"/scout-soroban/docs/vulnerabilities/assert-violation","docId":"vulnerabilities/assert-violation"}],"href":"/scout-soroban/docs/vulnerabilities/"},{"type":"category","label":"Detectors","collapsible":true,"collapsed":true,"items":[{"type":"link","label":"Divide before multiply","href":"/scout-soroban/docs/detectors/divide-before-multiply","docId":"detectors/divide-before-multiply"},{"type":"link","label":"Unsafe unwrap","href":"/scout-soroban/docs/detectors/unsafe-unwrap","docId":"detectors/unsafe-unwrap"},{"type":"link","label":"Unsafe expect","href":"/scout-soroban/docs/detectors/unsafe-expect","docId":"detectors/unsafe-expect"},{"type":"link","label":"Overflow-check","href":"/scout-soroban/docs/detectors/overflow-check","docId":"detectors/overflow-check"},{"type":"link","label":"Insuficciently random values","href":"/scout-soroban/docs/detectors/insufficiently-random-values","docId":"detectors/insufficiently-random-values"},{"type":"link","label":"Unprotected update of current contract wasm","href":"/scout-soroban/docs/detectors/unprotected-update-current-contract-wasm","docId":"detectors/unprotected-update-current-contract-wasm"},{"type":"link","label":"Avoid core mem forget usage","href":"/scout-soroban/docs/detectors/avoid-core-mem-forget","docId":"detectors/avoid-core-mem-forget"},{"type":"link","label":"Set contract storage","href":"/scout-soroban/docs/detectors/set-contract-storage","docId":"detectors/set-contract-storage"},{"type":"link","label":"Panic error","href":"/scout-soroban/docs/detectors/avoid-panic-error","docId":"detectors/avoid-panic-error"},{"type":"link","label":"Avoid unsafe block","href":"/scout-soroban/docs/detectors/avoid-unsafe-block","docId":"detectors/avoid-unsafe-block"},{"type":"link","label":"DoS unbounded operation","href":"/scout-soroban/docs/detectors/dos-unbounded-operation","docId":"detectors/dos-unbounded-operation"},{"type":"link","label":"Soroban version","href":"/scout-soroban/docs/detectors/soroban-version","docId":"detectors/soroban-version"},{"type":"link","label":"Unused return enum","href":"/scout-soroban/docs/detectors/unused-return-enum","docId":"detectors/unused-return-enum"},{"type":"link","label":"Iterators-over-indexing","href":"/scout-soroban/docs/detectors/iterators-over-indexing","docId":"detectors/iterators-over-indexing"},{"type":"link","label":"Assert  violation","href":"/scout-soroban/docs/detectors/assert-violation","docId":"detectors/assert-violation"},{"type":"link","label":"DoS unexpected revert with vector","href":"/scout-soroban/docs/detectors/dos-unexpected-revert-with-vector","docId":"detectors/dos-unexpected-revert-with-vector"},{"type":"link","label":"Unrestricted Transfer From","href":"/scout-soroban/docs/detectors/unrestricted-transfer-from","docId":"detectors/unrestricted-transfer-from"},{"type":"link","label":"Unsafe map get","href":"/scout-soroban/docs/detectors/unsafe-map-get","docId":"detectors/unsafe-map-get"}],"href":"/scout-soroban/docs/detectors/"},{"type":"link","label":"Contribute","href":"/scout-soroban/docs/contribute","docId":"contribute"},{"type":"link","label":"Architecture","href":"/scout-soroban/docs/architecture","docId":"architecture"},{"type":"category","label":"Precision and recall","collapsible":true,"collapsed":true,"items":[{"type":"link","label":"Scout Bug Fighter for Soroban: Improving Tool\'s Precision","href":"/scout-soroban/docs/precision-and-recall/first-iteration","docId":"precision-and-recall/first-iteration"}],"href":"/scout-soroban/docs/precision-and-recall/"},{"type":"link","label":"Scout GitHub Action","href":"/scout-soroban/docs/github-action","docId":"github-action"}]},"docs":{"architecture":{"id":"architecture","title":"Architecture","description":"Scout is built on Trail of Bits\u2019 Dylint, featuring a new set of lints. Dylint is a static analyzer that interfaces with the Rust compiler, providing access to the High-Level Intermediate Representation and the Mid-Level Intermediate Representation. These representations enable the accurate capture of many vulnerabilities. The lints are specifically designed to detect certain vulnerability classes. They are files integrated into the tool during compilation, and adding new lints, or detectors as we call them, is straightforward for any contributor. We have also contributed to the Dylint project, enhancing its capabilities to produce outputs in various formats, including PDF reports.","sidebar":"docsSidebar"},"contribute":{"id":"contribute","title":"Contribute","description":"Thank you for your interest in contributing to the development of new detectors.","sidebar":"docsSidebar"},"detectors/assert-violation":{"id":"detectors/assert-violation","title":"Assert  violation","description":"What it does\u200b","sidebar":"docsSidebar"},"detectors/avoid-core-mem-forget":{"id":"detectors/avoid-core-mem-forget","title":"Avoid core mem forget usage","description":"What it does","sidebar":"docsSidebar"},"detectors/avoid-panic-error":{"id":"detectors/avoid-panic-error","title":"Panic error","description":"What it does","sidebar":"docsSidebar"},"detectors/avoid-unsafe-block":{"id":"detectors/avoid-unsafe-block","title":"Avoid unsafe block","description":"What it does","sidebar":"docsSidebar"},"detectors/divide-before-multiply":{"id":"detectors/divide-before-multiply","title":"Divide before multiply","description":"What it does","sidebar":"docsSidebar"},"detectors/dos-unbounded-operation":{"id":"detectors/dos-unbounded-operation","title":"DoS unbounded operation","description":"What it does","sidebar":"docsSidebar"},"detectors/dos-unexpected-revert-with-vector":{"id":"detectors/dos-unexpected-revert-with-vector","title":"DoS unexpected revert with vector","description":"What it does","sidebar":"docsSidebar"},"detectors/insufficiently-random-values":{"id":"detectors/insufficiently-random-values","title":"Insuficciently random values","description":"What it does","sidebar":"docsSidebar"},"detectors/iterators-over-indexing":{"id":"detectors/iterators-over-indexing","title":"Iterators-over-indexing","description":"What it does","sidebar":"docsSidebar"},"detectors/overflow-check":{"id":"detectors/overflow-check","title":"Overflow-check","description":"What it does","sidebar":"docsSidebar"},"detectors/README":{"id":"detectors/README","title":"Detectors","description":"In this section we introduce our set of detectors powered by Dylint - a Rust linting tool.","sidebar":"docsSidebar"},"detectors/set-contract-storage":{"id":"detectors/set-contract-storage","title":"Set contract storage","description":"What it does","sidebar":"docsSidebar"},"detectors/soroban-version":{"id":"detectors/soroban-version","title":"Soroban version","description":"What it does","sidebar":"docsSidebar"},"detectors/unprotected-update-current-contract-wasm":{"id":"detectors/unprotected-update-current-contract-wasm","title":"Unprotected update of current contract wasm","description":"What it does","sidebar":"docsSidebar"},"detectors/unrestricted-transfer-from":{"id":"detectors/unrestricted-transfer-from","title":"Unrestricted Transfer From","description":"What it does","sidebar":"docsSidebar"},"detectors/unsafe-expect":{"id":"detectors/unsafe-expect","title":"Unsafe expect","description":"What it does","sidebar":"docsSidebar"},"detectors/unsafe-map-get":{"id":"detectors/unsafe-map-get","title":"Unsafe map get","description":"What it does","sidebar":"docsSidebar"},"detectors/unsafe-unwrap":{"id":"detectors/unsafe-unwrap","title":"Unsafe unwrap","description":"What it does","sidebar":"docsSidebar"},"detectors/unused-return-enum":{"id":"detectors/unused-return-enum","title":"Unused return enum","description":"What it does","sidebar":"docsSidebar"},"github-action":{"id":"github-action","title":"Scout GitHub Action","description":"At CoinFabrik, we understand the importance of ensuring code quality and security in every step of the development process. That\'s why we\'ve developed a GitHub action to integrate Scout into the CI/CD pipeline.","sidebar":"docsSidebar"},"intro":{"id":"intro","title":"Getting Started","description":"Let\'s discover Scout in less than 5 minutes!.","sidebar":"docsSidebar"},"precision-and-recall/first-iteration":{"id":"precision-and-recall/first-iteration","title":"Scout Bug Fighter for Soroban: Improving Tool\'s Precision","description":"In the scope of the second grant awarded to CoinFabrik by the Stellar Community Fund to advance the development of Scout for Soroban, the focus extends beyond incorporating new detectors and refining features. A key objective of this grant is to subject the tool to rigorous testing against real Soroban projects. Through this process, the aim is to analyze the outcomes meticulously, identifying areas for enhancement to increase the tool\'s precision. This includes minimizing false positives and false negatives, thereby fortifying its efficacy.","sidebar":"docsSidebar"},"precision-and-recall/README":{"id":"precision-and-recall/README","title":"Precision and recall","description":"This section outlines the tasks we perform to enhance the overall quality of Scout.","sidebar":"docsSidebar"},"vulnerabilities/assert-violation":{"id":"vulnerabilities/assert-violation","title":"Assert violation","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/avoid-core-mem-forget":{"id":"vulnerabilities/avoid-core-mem-forget","title":"Avoid core::mem::forget usage","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/avoid-unsafe-block":{"id":"vulnerabilities/avoid-unsafe-block","title":"Avoid unsafe block","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/divide-before-multiply":{"id":"vulnerabilities/divide-before-multiply","title":"Divide before multiply","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/insufficiently-random-values":{"id":"vulnerabilities/insufficiently-random-values","title":"Insufficiently random values","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/iterators-over-indexing":{"id":"vulnerabilities/iterators-over-indexing","title":"Iterators over indexing","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/overflow-check":{"id":"vulnerabilities/overflow-check","title":"Overflow check","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/README":{"id":"vulnerabilities/README","title":"Vulnerabilities","description":"This section lists relevant security-related issues typically introduced during the development of smart contracts. The list, though non-exhaustive, features highly relevant items. Each issue is assigned a severity label based on the taxonomy presented below.","sidebar":"docsSidebar"},"vulnerabilities/set-contract-storage":{"id":"vulnerabilities/set-contract-storage","title":"Set contract storage","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/soroban-version":{"id":"vulnerabilities/soroban-version","title":"Soroban version","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/unprotected-update-current-contract-wasm":{"id":"vulnerabilities/unprotected-update-current-contract-wasm","title":"Unprotected update current contract wasm","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/unsafe-expect":{"id":"vulnerabilities/unsafe-expect","title":"Unsafe expect","description":"Description","sidebar":"docsSidebar"},"vulnerabilities/unsafe-unwrap":{"id":"vulnerabilities/unsafe-unwrap","title":"Unsafe unwrap","description":"Description","sidebar":"docsSidebar"}}}')}}]);