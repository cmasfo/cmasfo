
# COCOMO

Constructive Cost Model

---

시스템의 비용을 산정하기 위해 시스템을 구성하고 있는 모듈과 서브 시스템의 비용 합계를 계산하는 방식이다.

1981년에 'Barry W. Boehm'이 초기 모델을 제안했고, 1995년에 COCOMO II로 확장했다.

먼저 완성될 시스템의 규모(lines of code)를 추정하고 이를 준비된 식에 대입하여 소요 인원/개월을 에측한다.

3가지 모델 유형: 모델의 복잡성에 따라 분류.

- Basic COCOMO
- Intermediate COCOMO
- Detailed COCOMO

3가지 프로젝트 유형: 프로젝트 유형을 구분하여 프로그램 규모(KDSI)와 프로그래머 M/M(Man/Month)의 관계 정의.

- Organic Projects
- Semi-detached Projects
- Embedded Projects

## Basic COCOMO

기본형 COCOMO

SW 개발 노력과 비용을 LOC(lines of code) 형태로 추정한 후 비용을 산정하는 '고정 단일값 모형(Static Single-valued Model)'

산정공식

- 개발 노력(MM)
	- a x (KDSI) ^ b
- 개발 기간(TDEV)
	- c x (MM) ^ d
- 적정 투입 인원(FPS)
	- MM / TDEV
- 인적 비용(COST)
	- MM x (인당 월평균 급여)

## Intermediate COCOMO

중간형 COCOMO

프로젝트 형태, 개발환경, 개발인력 요소에 따라 15개의 특성치를 적용한 방식.

- Product 속성 3가지
	- SW 신뢰도
	- DB 크기
	- 제품의 복잡성
- Computer 속성 4가지
	- 응답시간
	- 실행시간 제약
	- 기억장치 제약
	- 가상기계 환경의 휘발성
- Personal 속성 5가지
	- 분석가의 자질
	- 프로그래머의 자질
	- 응용분야의 경험
	- 컴퓨터와의 친숙성
	- 프로그래밍 언어 경험
- Project 속성 3가지
	- 개발 기간의 산정
	- 개발도구 사용
	- 방법론 응용

산정공식

- 개발노력(MM)
	- (Basic COCOMO의 MM) x (요인별 노력 승수)
- 개발기간(TDEV)
	- C X (MM) ^ d
- 적정투입인원(FPS)
	- MM / TDEV
- 인적비용(COST)
	- MM x (인당 월평균 급여)

## Detailed COCOMO

발전형 COCOMO

대형 시스템에 대하여 서브시스템의 상이한 특성을 반영하여 비용을 개별 산정한 후 합산하는 방식.

3단계 비용 산정

- 모듈
- 서브시스템
- 시스템 레벨

개발 단계별(SDLC)로 비용 산정방식을 달리 할 수 있음

산정공식

- 중간형 COCOMO 산정 공식 그대로 사용
- 노력 승수 = 개발 공정별 노력 승수 x 개발 공정별 가중치

## Organic Mode

조직형

- 5만 LoC 이하
- 소규모 팀이 개발하는 잘 알려진 응용 시스템
- PM = 2.4 x (KDSI)\^1.05

## Semi-detached Mode

반-분리형

- 30만 LoC 이하
- 트랜잭션 처리, 운영체제, DBMS 등
- PM = 3.0 x (KDSI)\^1.12

## Embedded Mode

내장형

- 30만 LoC 이상
- 하드웨어가 포함된 최상위 규모의 실시간 처리 시스템
- 미사일 유도, 신호기 제어 시스템 등
- PM = 3.6 x (KDSI)\^1.20

---
