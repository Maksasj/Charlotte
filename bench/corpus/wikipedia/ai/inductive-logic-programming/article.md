# Inductive logic programming

Inductive logic programming (ILP) is a subfield of symbolic artificial intelligence which uses logic programming as a uniform representation for examples, background knowledge and hypotheses. The term "inductive" here refers to philosophical (i.e. suggesting a theory to explain observed facts) rather than mathematical (i.e. proving a property for all members of a well-ordered set) induction. Given an encoding of the known background knowledge and a set of examples represented as a logical database of facts, an ILP system will derive a hypothesised logic program which entails all the positive and none of the negative examples.

Schema: positive examples + negative examples + background knowledge ⇒ hypothesis.

Bioinformatics and drug design have been highlighted as a principal application area of inductive logic programming techniques.

## History

Building on earlier work on Inductive inference, Gordon Plotkin was the first to formalise induction in a clausal setting around 1970, adopting an approach of generalising from examples. In 1981, Ehud Shapiro introduced several ideas that would shape the field in his new approach of model inference, an algorithm employing refinement and backtracing to search for a complete axiomatisation of given examples. His first implementation was the Model Inference System in 1981: a Prolog program that inductively inferred Horn clause logic programs from positive and negative examples. The term Inductive Logic Programming was first introduced in a paper by Stephen Muggleton in 1990, defined as the intersection of machine learning and logic programming. Muggleton and Wray Buntine introduced predicate invention and inverse resolution in 1988.

Several inductive logic programming systems that proved influential appeared in the early 1990s. FOIL, introduced by Ross Quinlan in 1990 was based on upgrading propositional learning algorithms AQ and ID3. Golem, introduced by Muggleton and Feng in 1990, went back to a restricted form of Plotkin's least generalisation algorithm. The Progol system, introduced by Muggleton in 1995, first implemented inverse entailment, and inspired many later systems. Aleph, a descendant of Progol introduced by Ashwin Srinivasan in 2001, is still one of the most widely used systems as of 2022.

At around the same time, the first practical applications emerged, particularly in bioinformatics, where by 2000 inductive logic programming had been successfully applied to drug design, carcinogenicity and mutagenicity prediction, and elucidation of the structure and function of proteins. Unlike the focus on automatic programming inherent in the early work, these fields used inductive logic programming techniques from a viewpoint of relational data mining. The success of those initial applications and the lack of progress in recovering larger traditional logic programs shaped the focus of the field.

Recently, classical tasks from automated programming have moved back into focus, as the introduction of meta-interpretative learning makes predicate invention and learning recursive programs more feasible. This technique was pioneered with the Metagol system introduced by Muggleton, Dianhuan Lin, Niels Pahlavi and Alireza Tamaddoni-Nezhad in 2014. This allows ILP systems to work with fewer examples, and brought successes in learning string transformation programs, answer set grammars and general algorithms.

## Setting

Inductive logic programming has adopted several different learning settings, the most common of which are learning from entailment and learning from interpretations. In both cases, the input is provided in the form of background knowledge B, a logical theory (commonly in the form of clauses used in logic programming), as well as positive and negative examples, denoted E+ and E− respectively. The output is given as a hypothesis H, itself a logical theory that typically consists of one or more clauses.

The two settings differ in the format of examples presented.

### Learning from entailment

As of 2022, learning from entailment is by far the most popular setting for inductive logic programming. In this setting, the positive and negative examples are given as finite sets E+ and E− of positive and negated ground literals, respectively. A correct hypothesis H is a set of clauses satisfying the following requirements, where the turnstile symbol ⊨ stands for logical entailment:

Completeness:B∪H⊨E+Consistency: B∪H∪E−⊭𝑓𝑎𝑙𝑠𝑒

Completeness requires any generated hypothesis H to explain all positive examples E+, and consistency forbids generation of any hypothesis H that is inconsistent with the negative examples E−, both given the background knowledge B.

In Muggleton's setting of concept learning, "completeness" is referred to as "sufficiency", and "consistency" as "strong consistency". Two further conditions are added: "Necessity", which postulates that B does not entail E+, does not impose a restriction on H, but forbids any generation of a hypothesis as long as the positive facts are explainable without it. "Weak consistency", which states that no contradiction can be derived from B∧H, forbids generation of any hypothesis H that contradicts the background knowledge B. Weak consistency is implied by strong consistency; if no negative examples are given, both requirements coincide. Weak consistency is particularly important in the case of noisy data, where completeness and strong consistency cannot be guaranteed.

### Learning from interpretations

In learning from interpretations, the positive and negative examples are given as a set of complete or partial Herbrand structures, each of which are themselves a finite set of ground literals. Such a structure e is said to be a model of the set of clauses B∪H if for any substitution θ and any clause head←body in B∪H such that bodyθ⊆e, headθ⊆e also holds. The goal is then to output a hypothesis that is complete, meaning every positive example is a model of B∪H, and consistent, meaning that no negative example is a model of B∪H.

## Approaches to ILP

An inductive logic programming system is a program that takes as an input logic theories B,E+,E− and outputs a correct hypothesis H with respect to theories B,E+,E−. A system is complete if and only if for any input logic theories B,E+,E− any correct hypothesis H with respect to these input theories can be found with its hypothesis search procedure. Inductive logic programming systems can be roughly divided into two classes, search-based and meta-interpretative systems.

Search-based systems exploit that the space of possible clauses forms a complete lattice under the subsumption relation, where one clause C1 subsumes another clause C2 if there is a substitution θ such that C1θ, the result of applying θ to C1, is a subset of C2. This lattice can be traversed either bottom-up or top-down.

### Bottom-up search

Bottom-up methods to search the subsumption lattice have been investigated since Plotkin's first work on formalising induction in clausal logic in 1970. Techniques used include least general generalisation, based on anti-unification, and inverse resolution, based on inverting the resolution inference rule.

#### Least general generalisation

A least general generalisation algorithm takes as input two clauses C1 and C2 and outputs the least general generalisation of C1 and C2, that is, a clause C that subsumes C1 and C2, and that is subsumed by every other clause that subsumes C1 and C2. The least general generalisation can be computed by first computing all selections from C1 and C2, which are pairs of literals (L,M)∈(C1×C2) sharing the same predicate symbol and negated/unnegated status. Then, the least general generalisation is obtained as the disjunction of the least general generalisations of the individual selections, which can be obtained by first-order syntactical anti-unification.

To account for background knowledge, inductive logic programming systems employ relative least general generalisations, which are defined in terms of subsumption relative to a background theory. In general, such relative least general generalisations are not guaranteed to exist; however, if the background theory B is a finite set of ground literals, then the negation of B is itself a clause. In this case, a relative least general generalisation can be computed by disjoining the negation of B with both C1 and C2 and then computing their least general generalisation as before.

Relative least general generalisations are the foundation of the bottom-up system Golem.

#### Inverse resolution

Inverse resolution is an inductive reasoning technique that involves inverting the resolution operator.

Inverse resolution takes information about the resolvent of a resolution step to compute possible resolving clauses. Two types of inverse resolution operator are in use in inductive logic programming: V-operators and W-operators. A V-operator takes clauses R and C1as input and returns a clause C2 such that R is the resolvent of C1 and C2. A W-operator takes two clauses R1 and R2 and returns three clauses C1, C2 and C3 such that R1 is the resolvent of C1 and C2 and R2 is the resolvent of C2 and C3.

Inverse resolution was first introduced by Stephen Muggleton and Wray Buntine in 1988 for use in the inductive logic programming system Cigol. By 1993, this spawned a surge of research into inverse resolution operators and their properties.

### Top-down search

The ILP systems Progol, Hail  and Imparo  find a hypothesis H using the principle of the inverse entailment for theories B, E, H: B∧H⊨E⟺B∧¬E⊨¬H. First they construct an intermediate theory F called a bridge theory satisfying the conditions B∧¬E⊨F and F⊨¬H. Then as H⊨¬F, they generalize the negation of the bridge theory F with anti-entailment. However, the operation of anti-entailment is computationally more expensive since it is highly nondeterministic. Therefore, an alternative hypothesis search can be conducted using the inverse subsumption (anti-subsumption) operation instead, which is less non-deterministic than anti-entailment.

Questions of completeness of a hypothesis search procedure of specific inductive logic programming system arise. For example, the Progol hypothesis search procedure based on the inverse entailment inference rule is not complete by Yamamoto's example. On the other hand, Imparo is complete by both anti-entailment procedure  and its extended inverse subsumption  procedure.

### Metainterpretive learning

Rather than explicitly searching the hypothesis graph, metainterpretive or meta-level systems encode the inductive logic programming program as a meta-level logic program which is then solved to obtain an optimal hypothesis. Formalisms used to express the problem specification include Prolog and answer set programming, with existing Prolog systems and answer set solvers used for solving the constraints.

And example of a Prolog-based system is Metagol, which is based on a meta-interpreter in Prolog, while ASPAL and ILASP are based on an encoding of the inductive logic programming problem in answer set programming.

### Evolutionary learning

Evolutionary algorithms in ILP use a population-based approach to evolve hypotheses, refining them through selection, crossover, and mutation. Methods like EvoLearner have been shown to outperform traditional approaches on structured machine learning benchmarks.

## List of implementations

1BC and 1BC2: first-order naive Bayesian classifiers:

ACE (A Combined Engine)

Aleph

Atom Archived 2014-03-26 at the Wayback Machine

Claudien[link removed]

DL-Learner Archived 2019-08-15 at the Wayback Machine

DMax

FastLAS (Fast Learning from Answer Sets)

FOIL (First Order Inductive Learner)

Golem

ILASP (Inductive Learning of Answer Set Programs)

Imparo

Inthelex (INcremental THEory Learner from EXamples) Archived 2011-11-28 at the Wayback Machine

Lime

Metagol

Mio[link removed]

MIS (Model Inference System) by Ehud Shapiro

Ontolearn

Popper

PROGOL

RSD

Warmr (now included in ACE)

ProGolem

## Probabilistic inductive logic programming

Probabilistic inductive logic programming adapts the setting of inductive logic programming to learning probabilistic logic programs. It can be considered as a form of statistical relational learning within the formalism of probabilistic logic programming.

Given

background knowledge as a probabilistic logic program B, and

a set of positive and negative examples E+ and E−

the goal of probabilistic inductive logic programming is to find a probabilistic logic program H such that the probability of positive examples according to H∪B is maximized and the probability of negative examples is minimized.

This problem has two variants: parameter learning and structure learning. In the former, one is given the structure (the clauses) of H and the goal is to infer the probabilities annotations of the given clauses, while in the latter the goal is to infer both the structure and the probability parameters of H. Just as in classical inductive logic programming, the examples can be given as examples or as (partial) interpretations.

### Parameter Learning

Parameter learning for languages following the distribution semantics has been performed by using an expectation-maximisation algorithm or by gradient descent.

An expectation-maximisation algorithm consists of a cycle in which the steps of expectation and maximization are repeatedly performed. In the expectation step, the distribution of the hidden variables is computed according to the current values of the probability parameters, while in the maximisation step, the new values of the parameters are computed.

Gradient descent methods compute the gradient of the target function and iteratively modify the parameters moving in the direction of the gradient.

### Structure Learning

Structure learning was pioneered by Daphne Koller and Avi Pfeffer in 1997, where the authors learn the structure of first-order rules with associated probabilistic uncertainty parameters. Their approach involves generating the underlying graphical model in a preliminary step and then applying expectation-maximisation.

In 2008, De Raedt et al. presented an algorithm for performing theory compression on ProbLog programs, where theory compression refers to a process of removing as many clauses as possible from the theory in order to maximize the probability of a given set of positive and negative examples. No new clause can be added to the theory.

In the same year, Meert, W. et al. introduced a method for learning parameters and structure of ground probabilistic logic programs by considering the Bayesian networks equivalent to them and applying techniques for learning Bayesian networks.

ProbFOIL, introduced by De Raedt and Ingo Thon in 2010, combined the inductive logic programming system FOIL with ProbLog. Logical rules are learned from probabilistic data in the sense that both the examples themselves and their classifications can be probabilistic. The set of rules has to allow one to predict the probability of the examples from their description. In this setting, the parameters (the probability values) are fixed and the structure has to be learned.

In 2011, Elena Bellodi and Fabrizio Riguzzi introduced SLIPCASE, which performs a beam search among probabilistic logic programs by iteratively refining probabilistic theories and optimizing the parameters of each theory using expectation-maximisation.

Its extension SLIPCOVER, proposed in 2014, uses bottom clauses generated as in Progol to guide the refinement process, thus reducing the number of revisions and exploring the search space more effectively. Moreover, SLIPCOVER separates the search for promising clauses from that of the theory: the space of clauses is explored with a beam search, while the space of theories is searched greedily.
