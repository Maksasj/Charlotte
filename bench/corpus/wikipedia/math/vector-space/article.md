# Vector space

In mathematics, a vector space (also called a linear space) is a set whose elements, often called vectors, can be added together and multiplied ("scaled") by numbers called scalars. The operations of vector addition and scalar multiplication must satisfy certain requirements, called vector axioms. Real vector spaces and complex vector spaces are kinds of vector spaces based on different kinds of scalars: real numbers and complex numbers. Scalars can also be, more generally, elements of any field.

Vector spaces generalize Euclidean vectors, which allow modeling of physical quantities (such as forces and velocity) that have not only a magnitude, but also a direction. The concept of vector spaces is fundamental for linear algebra, together with the concept of matrices, which allows computing in vector spaces. This provides a concise and synthetic way for manipulating and studying systems of linear equations.

Vector spaces are characterized by their dimension, which, roughly speaking, specifies the number of independent directions in the space. This means that for two vector spaces over a given field and with the same dimension, the properties that depend only on the vector-space structure are exactly the same (that is, the vector spaces are isomorphic). A vector space is finite-dimensional if its dimension is a natural number. Otherwise, it is infinite-dimensional, and its dimension is an infinite cardinal. Finite-dimensional vector spaces occur naturally in geometry and related areas. Infinite-dimensional vector spaces occur in many areas of mathematics. For example, polynomial rings are countably infinite-dimensional vector spaces, and many function spaces have the cardinality of the continuum as a dimension.

Many vector spaces that are considered in mathematics are also endowed with other structures. This is the case of algebras, which include field extensions, polynomial rings, associative algebras and Lie algebras. This is also the case of topological vector spaces, which include function spaces, inner product spaces, normed spaces, Hilbert spaces and Banach spaces.

## Definition and basic properties

In this article, vectors are represented in boldface to distinguish them from scalars.

A vector space over a field F is a non-empty set V together with a binary operation and a binary function that satisfy the eight axioms listed below. In this context, the elements of V are commonly called vectors, and the elements of F are called scalars.

The binary operation, called vector addition or simply addition assigns to any two vectors v and w in V a third vector in V which is commonly written as v + w, and called the sum of these two vectors.

The binary function, called scalar multiplication, assigns to any scalar a in F and any vector v in V another vector in V, which is denoted av.

To have a vector space, the eight following axioms must be satisfied for every u, v and w in V, and a and b in F.

When the scalar field is the real numbers, the vector space is called a real vector space, and when the scalar field is the complex numbers, the vector space is called a complex vector space. These two cases are the most common ones, but vector spaces with scalars in an arbitrary field F are also commonly considered. Such a vector space is called an F-vector space or a vector space over F.

An equivalent definition of a vector space can be given, which is much more concise but less elementary: "a vector field is an abelian group under addition with a ring homomorphism from the field F into the endomorphism ring of this group". Indeed, the first four axioms (related to vector addition) say that a vector space is an abelian group under addition, the distributivity of scalar multiplication with respect to vector addition means that multiplication by a scalar a is an endomorphism of the group, and the remaining three axioms establish that the function that maps a scalar a to the multiplication by a is  a ring homomorphism from the field to the endomorphism ring of the group.

Subtraction of two vectors can be defined as

𝐯−𝐰=𝐯+(−𝐰).

Direct consequences of the axioms include that, for every s∈F and 𝐯∈V, one has

0𝐯=𝟎,

s𝟎=𝟎,

(−1)𝐯=−𝐯,

s𝐯=𝟎 implies s=0 or 𝐯=𝟎.

Even more concisely, a vector space is a module over a field.

## Bases, vector coordinates, and subspaces

Linear combination

Given a set G of elements of a F-vector space V, a linear combination of elements of G is an element of V of the form a1𝐠1+a2𝐠2+⋯+ak𝐠k, where a1,…,ak∈F and 𝐠1,…,𝐠k∈G. The scalars a1,…,ak are called the coefficients of the linear combination.

Linear independence

The elements of a subset G of a F-vector space V are said to be linearly independent if no element of G can be written as a linear combination of the other elements of G. Equivalently, they are linearly independent if two linear combinations of elements of G define the same element of V if and only if they have the same coefficients. Also equivalently, they are linearly independent if a linear combination results in the zero vector if and only if all its coefficients are zero.

Linear subspace

A linear subspace or vector subspace W of a vector space V is a non-empty subset of V that is closed under vector addition and scalar multiplication; that is, the sum of two elements of W and the product of an element of W by a scalar belong to W. This implies that every linear combination of elements of W belongs to W. A linear subspace is a vector space for the induced addition and scalar multiplication; this means that the closure property implies that the axioms of a vector space are satisfied.The closure property also implies that every intersection of linear subspaces is a linear subspace.

Linear span

Given a subset G of a vector space V, the linear span or simply the span of G is the smallest linear subspace of V that contains G, in the sense that it is the intersection of all linear subspaces that contain G. The span of G is also the set of all linear combinations of elements of G. If W is the span of G, one says that G spans or generates W, and that G is a spanning set or a generating set of W.

Basis and dimension

A subset of a vector space is a basis if its elements are linearly independent and span the vector space. Every vector space has at least one basis, or many in general (see Basis (linear algebra) § Proof that every vector space has a basis). Moreover, all bases of a vector space have the same cardinality, which is called the dimension of the vector space (see Dimension theorem for vector spaces). This is a fundamental property of vector spaces, which is detailed in the remainder of the section.

Bases are a fundamental tool for the study of vector spaces, especially when the dimension is finite. In the infinite-dimensional case, the existence of infinite bases, often called Hamel bases, depends on the axiom of choice. It follows that, in general, no base can be explicitly described. For example, the real numbers form an infinite-dimensional vector space over the rational numbers, for which no specific basis is known.

Consider a basis (𝐛1,𝐛2,…,𝐛n) of a vector space V of dimension n over a field F. The definition of a basis implies that every 𝐯∈V may be written

𝐯=a1𝐛1+⋯+an𝐛n,

with a1,…,an in F, and that this decomposition is unique. The scalars a1,…,an are called the coordinates of v on the basis. They are also said to be the coefficients of the decomposition of v on the basis. One also says that the n-tuple of the coordinates is the coordinate vector of v on the basis, since the set Fn of the n-tuples of elements of F is a vector space for componentwise addition and scalar multiplication, whose dimension is n.

The one-to-one correspondence between vectors and their coordinate vectors maps vector addition to vector addition and scalar multiplication to scalar multiplication. It is thus a vector space isomorphism, which allows translating reasonings and computations on vectors into reasonings and computations on their coordinates.

## History

Vector spaces stem from affine geometry, via the introduction of coordinates in the plane or three-dimensional space. Around 1636, French mathematicians René Descartes and Pierre de Fermat founded analytic geometry by identifying solutions to an equation of two variables with points on a plane curve. To achieve geometric solutions without using coordinates, Bolzano introduced, in 1804, certain operations on points, lines, and planes, which are predecessors of vectors. Möbius (1827) introduced the notion of barycentric coordinates. Bellavitis (1833) introduced an equivalence relation on directed line segments that share the same length and direction which he called equipollence. A Euclidean vector is then an equivalence class of that relation.

Vectors were reconsidered with the presentation of complex numbers by Argand and Hamilton and the inception of quaternions by the latter. They are elements in R2 and R4; treating them using linear combinations goes back to Laguerre in 1867, who also defined systems of linear equations.

In 1857, Cayley introduced the matrix notation which allows for harmonization and simplification of linear maps. Around the same time, Grassmann studied the barycentric calculus initiated by Möbius. He envisaged sets of abstract objects endowed with operations. In his work, the concepts of linear independence and dimension, as well as scalar products are present. Grassmann's 1844 work exceeds the framework of vector spaces as well since his considering multiplication led him to what are today called algebras. Italian mathematician Peano was the first to give the modern definition of vector spaces and linear maps in 1888, although he called them "linear systems". Peano's axiomatization allowed for vector spaces with infinite dimension, but Peano did not develop that theory further. In 1897, Salvatore Pincherle adopted Peano's axioms and made initial inroads into the theory of infinite-dimensional vector spaces.

An important development of vector spaces is due to the construction of function spaces by Henri Lebesgue. This was later formalized by Banach and Hilbert, around 1920. At that time, algebra and the new field of functional analysis began to interact, notably with key concepts such as spaces of p-integrable functions and Hilbert spaces.

### Arrows in the plane

The first example of a vector space consists of "arrows" in a fixed plane, starting at one fixed point. This corresponds to Euclidean vectors, as used in physics to describe forces or velocities. Given any two such arrows, v and w, the parallelogram spanned by these two arrows contains one diagonal arrow that starts at the origin, too. This new arrow is called the sum of the two arrows, and is denoted v + w. In the special case of two arrows on the same line, their sum is the arrow on this line whose length is the sum or the difference of the lengths, depending on whether the arrows have the same direction. Another operation that can be done with arrows is scaling: given any positive real number a, the arrow that has the same direction as v, but is dilated or shrunk by multiplying its length by a, is called multiplication of v by a. It is denoted av. When a is negative, av is defined as the arrow pointing in the opposite direction instead.

The following shows a few examples: if a = 2, the resulting vector aw has the same direction as w, but is stretched to the double length of w (the second image). Equivalently, 2w is the sum w + w. Moreover, (−1)v = −v has the opposite direction and the same length as v (blue vector pointing down in the second image).

### Ordered pairs of numbers

A second key example of a vector space is provided by pairs of real numbers x and y. The order of the components x and y is significant, so such a pair is also called an ordered pair. Such a pair is written as (x, y). The sum of two such pairs and the multiplication of a pair with a number is defined as follows:

(x1,y1)+(x2,y2)=(x1+x2,y1+y2),a(x,y)=(ax,ay).

The first example above reduces to this example if an arrow is represented by a pair of Cartesian coordinates of its endpoint.

### Coordinate space

The simplest example of a vector space over a field F is the field F itself with its addition viewed as vector addition and its multiplication viewed as scalar multiplication. More generally, all n-tuples (sequences of length n)

(a1,a2,…,an)

of elements ai of F form a vector space that is usually denoted Fn and called a coordinate space.

The case n = 1 is the above-mentioned simplest example, in which the field F is also regarded as a vector space over itself. The case F = R and n = 2 (so R2) reduces to the previous example.

### Complex numbers and other field extensions

The set of complex numbers C, numbers that can be written in the form x + iy for real numbers x and y where i is the imaginary unit, form a vector space over the reals with the usual addition and multiplication: (x + iy) + (a + ib) = (x + a) + i(y + b) and c ⋅ (x + iy) = (c ⋅ x) + i(c ⋅ y) for real numbers x, y, a, b and c.  The various axioms of a vector space follow from the fact that the same rules hold for complex number arithmetic. The example of complex numbers is essentially the same as (that is, it is isomorphic to) the vector space of ordered pairs of real numbers mentioned above: if we think of the complex number x + i y as representing the ordered pair (x, y) in the complex plane then we see that the rules for addition and scalar multiplication correspond exactly to those in the earlier example.

More generally, field extensions provide another class of examples of vector spaces, particularly in algebra and algebraic number theory: a field F containing a smaller field E is an E-vector space, by the given multiplication and addition operations of F. For example, the complex numbers are a vector space over R, and the field extension 𝐐(i5) is a vector space over Q.

### Function spaces

Functions from any fixed set Ω to a field F also form vector spaces, by performing addition and scalar multiplication pointwise. That is, the sum of two functions f and g is the function (f+g) given by

(f+g)(w)=f(w)+g(w),

and similarly for multiplication. Such function spaces occur in many geometric situations, when Ω is the real line or an interval, or other subsets of R. Many notions in topology and analysis, such as continuity, integrability or differentiability are well-behaved with respect to linearity: sums and scalar multiples of functions possessing such a property still have that property. Therefore, the set of such functions are vector spaces, whose study belongs to functional analysis.

### Linear equations

Systems of homogeneous linear equations are closely tied to vector spaces. For example, the solutions of

a+3b+c=04a+2b+2c=0

are given by triples with arbitrary a, b=a/2, and c=−5a/2. They form a vector space: sums and scalar multiples of such triples still satisfy the same ratios of the three variables; thus they are solutions, too. Matrices can be used to condense multiple linear equations as above into one vector equation, namely

where A=[131422] is the matrix containing the coefficients of the given equations, 𝐱 is the vector (a,b,c), A𝐱 denotes the matrix product, and 𝟎=(0,0) is the zero vector. In a similar vein, the solutions of homogeneous linear differential equations form vector spaces. For example,

yields f(x)=ae−x+bxe−x, where a and b are arbitrary constants, and ex is the natural exponential function.

## Linear maps and matrices

The relation of two vector spaces can be expressed by linear map or linear transformation. They are functions that reflect the vector space structure, that is, they preserve sums and scalar multiplication:

f(𝐯+𝐰)=f(𝐯)+f(𝐰),f(a⋅𝐯)=a⋅f(𝐯)

for all 𝐯 and 𝐰 in V, all a in F.

An isomorphism is a linear map f : V → W such that there exists an inverse map g : W → V, which is a map such that the two possible compositions f ∘ g : W → W and g ∘ f : V → V are identity maps. Equivalently, f is both one-to-one (injective) and onto (surjective). If there exists an isomorphism between V and W, the two spaces are said to be isomorphic; they are then essentially identical as vector spaces, since all identities holding in V are, via f, transported to similar ones in W, and vice versa via g.

For example, the arrows in the plane and the ordered pairs of numbers vector spaces in the introduction above (see § Examples) are isomorphic: a planar arrow v departing at the origin of some (fixed) coordinate system can be expressed as an ordered pair by considering the x- and y-component of the arrow, as shown in the image at the right. Conversely, given a pair (x, y), the arrow going by x to the right (or to the left, if x is negative), and y up (down, if y is negative) turns back the arrow v.

Linear maps V → W between two vector spaces form a vector space HomF(V, W), also denoted L(V, W), or 𝓛(V, W). The space of linear maps from V to F is called the dual vector space, denoted V∗. Via the injective natural map V → V∗∗, any vector space can be embedded into its bidual; the map is an isomorphism if and only if the space is finite-dimensional.

Once a basis of V is chosen, linear maps f : V → W are completely determined by specifying the images of the basis vectors, because any element of V is expressed uniquely as a linear combination of them. If dim V = dim W, a 1-to-1 correspondence between fixed bases of V and W gives rise to a linear map that maps any basis element of V to the corresponding basis element of W. It is an isomorphism, by its very definition. Therefore, two vector spaces over a given field are isomorphic if their dimensions agree and vice versa. Another way to express this is that any vector space over a given field is completely classified (up to isomorphism) by its dimension, a single number. In particular, any n-dimensional F-vector space V is isomorphic to Fn. However, there is no "canonical" or preferred isomorphism; an isomorphism φ : Fn → V is equivalent to the choice of a basis of V, by mapping the standard basis of Fn to V, via φ.

### Matrices

Matrices are a useful notion to encode linear maps. They are written as a rectangular array of scalars as in the image at the right. Any m-by-n matrix A gives rise to a linear map from Fn to Fm, by the following

𝐱=(x1,x2,…,xn)↦(∑j=1na1jxj,∑j=1na2jxj,…,∑j=1namjxj),

where ∑ denotes summation, or by using the matrix multiplication of the matrix A with the coordinate vector 𝐱:

Moreover, after choosing bases of V and W, any linear map f : V → W is uniquely represented by a matrix via this assignment.

The determinant det (A) of a square matrix A is a scalar that tells whether the associated map is an isomorphism or not: to be so it is sufficient and necessary that the determinant is nonzero. The linear transformation of Rn corresponding to a real n-by-n matrix is orientation preserving if and only if its determinant is positive.

### Eigenvalues and eigenvectors

Endomorphisms, linear maps f : V → V, are particularly important since in this case vectors v can be compared with their image under f, f(v). Any nonzero vector v satisfying λv = f(v), where λ is a scalar, is called an eigenvector of f with eigenvalue λ. Equivalently, v is an element of the kernel of the difference f − λ · Id (where Id is the identity map V → V). If V is finite-dimensional, this can be rephrased using determinants: f  having eigenvalue λ is equivalent to

det⁡(f−λ⋅Id⁡)=0.

By spelling out the definition of the determinant, the expression on the left hand side can be seen to be a polynomial function in λ, called the characteristic polynomial of f. If the field F is large enough to contain a zero of this polynomial (which automatically happens for F algebraically closed, such as F = C) any linear map has at least one eigenvector. The vector space V may or may not possess an eigenbasis, a basis consisting of eigenvectors. This phenomenon is governed by the Jordan canonical form of the map. The set of all eigenvectors corresponding to a particular eigenvalue of f forms a vector space known as the eigenspace corresponding to the eigenvalue (and f) in question.

## Basic constructions

In addition to the above concrete examples, there are a number of standard linear algebraic constructions that yield vector spaces related to given ones.

### Subspaces and quotient spaces

A nonempty subset W of a vector space V that is closed under addition and scalar multiplication (and therefore contains the 𝟎-vector of V) is called a linear subspace of V, or simply a subspace of V, when the ambient space is unambiguously a vector space. Subspaces of V are vector spaces (over the same field) in their own right. The intersection of all subspaces containing a given set S of vectors is called its span, and it is the smallest subspace of V containing the set S. Expressed in terms of elements, the span is the subspace consisting of all the linear combinations of elements of S.

Linear subspace of dimension 1 and 2 are referred to as a line (also vector line), and a plane respectively. If W is an n-dimensional vector space, any subspace of dimension 1 less, i.e., of dimension n−1 is called a hyperplane.

The counterpart to subspaces are quotient vector spaces. Given any subspace W⊆V, the quotient space V/W ("V modulo W") is defined as follows: as a set, it consists of

𝐯+W={𝐯+𝐰:𝐰∈W},

where 𝐯 is an arbitrary vector in V. The sum of two such elements 𝐯1+W and 𝐯2+W is (𝐯1+𝐯2)+W, and scalar multiplication is given by a⋅(𝐯+W)=(a⋅𝐯)+W. The key point in this definition is that 𝐯1+W=𝐯2+W if and only if the difference of 𝐯1 and 𝐯2 lies in W. This way, the quotient space "forgets" information that is contained in the subspace W.

The kernel ker⁡(f) of a linear map f:V→W consists of vectors 𝐯 that are mapped to 𝟎 in W. The kernel and the image im⁡(f)={f(𝐯):𝐯∈V} are subspaces of V and W, respectively.

An important example is the kernel of a linear map 𝐱↦A𝐱 for some fixed matrix A. The kernel of this map is the subspace of vectors 𝐱 such that A𝐱=𝟎, which is precisely the set of solutions to the system of homogeneous linear equations belonging to A. This concept also extends to linear differential equations

a0f+a1dfdx+a2d2fdx2+⋯+andnfdxn=0,

where the coefficients ai are functions in x, too.

In the corresponding map

f↦D(f)=∑i=0naidifdxi,

the derivatives of the function f appear linearly (as opposed to f″(x)2, for example). Since differentiation is a linear procedure (that is, (f+g)′=f′+g′ and (c⋅f)′=c⋅f′ for a constant c) this assignment is linear, called a linear differential operator. In particular, the solutions to the differential equation D(f)=0 form a vector space (over R or C).

The existence of kernels and images is part of the statement that the category of vector spaces (over a fixed field F) is an abelian category, that is, a corpus of mathematical objects and structure-preserving maps between them (a category) that behaves much like the category of abelian groups. Because of this, many statements such as the first isomorphism theorem (also called rank–nullity theorem in matrix-related terms)

V/ker⁡(f)≡im⁡(f)

and the second and third isomorphism theorem can be formulated and proven in a way very similar to the corresponding statements for groups.

### Direct product and direct sum

The direct product of vector spaces and the direct sum of vector spaces are two ways of combining an indexed family of vector spaces into a new vector space.

The direct product ∏i∈IVi of a family of vector spaces Vi consists of the set of all tuples (𝐯i)i∈I, which specify for each index i in some index set I an element 𝐯i of Vi. Addition and scalar multiplication is performed componentwise. A variant of this construction is the direct sum ⨁i∈IVi (also called coproduct and denoted ∐i∈IVi), where only tuples with finitely many nonzero vectors are allowed. If the index set I is finite, the two constructions agree, but in general they are different.

### Tensor product

The tensor product V⊗FW, or simply V⊗W, of two vector spaces V and W is one of the central notions of multilinear algebra, which deals with extending notions such as linear maps to several variables. A map g:V×W→X from the Cartesian product V×W is called bilinear if g is linear in both variables 𝐯 and 𝐰.  That is to say, for fixed 𝐰 the map 𝐯↦g(𝐯,𝐰) is linear in the sense above and likewise for fixed 𝐯.

The tensor product is a particular vector space that is a universal recipient of bilinear maps g, as follows. It is defined as the vector space consisting of finite (formal) sums of symbols called tensors

𝐯1⊗𝐰1+𝐯2⊗𝐰2+⋯+𝐯n⊗𝐰n,

subject to the rules

a⋅(𝐯⊗𝐰) = (a⋅𝐯)⊗𝐰 = 𝐯⊗(a⋅𝐰),   where a is a scalar(𝐯1+𝐯2)⊗𝐰 = 𝐯1⊗𝐰+𝐯2⊗𝐰𝐯⊗(𝐰1+𝐰2) = 𝐯⊗𝐰1+𝐯⊗𝐰2.

These rules ensure that the map f from the V×W to V⊗W that maps a tuple (𝐯,𝐰) to 𝐯⊗𝐰 is bilinear. The universality states that given any vector space X and any bilinear map g:V×W→X, there exists a unique map u, shown in the diagram with a dotted arrow, whose composition with f equals g: u(𝐯⊗𝐰)=g(𝐯,𝐰). This is called the universal property of the tensor product, an instance of the method—much used in advanced abstract algebra—to indirectly define objects by specifying maps from or to this object.

## Vector spaces with additional structure

From the point of view of linear algebra, vector spaces are completely understood insofar as any vector space over a given field is characterized, up to isomorphism, by its dimension. However, vector spaces per se do not offer a framework to deal with the question—crucial to analysis—whether a sequence of functions converges to another function. Likewise, linear algebra is not adapted to deal with infinite series, since the addition operation allows only finitely many terms to be added. Therefore, the needs of functional analysis require considering additional structures.

A vector space may be given a partial order ≤, under which some vectors can be compared. For example, n-dimensional real space 𝐑n can be ordered by comparing its vectors componentwise. Ordered vector spaces, for example Riesz spaces, are fundamental to Lebesgue integration, which relies on the ability to express a function as a difference of two positive functions

f=f+−f−.

where f+ denotes the positive part of f and f− the negative part.

### Normed vector spaces and inner product spaces

"Measuring" vectors is done by specifying a norm, a datum which measures lengths of vectors, or by an inner product, which measures angles between vectors. Norms and inner products are denoted |𝐯| and ⟨𝐯,𝐰⟩, respectively. The datum of an inner product entails that lengths of vectors can be defined too, by defining the associated norm |𝐯|:=⟨𝐯,𝐯⟩. Vector spaces endowed with such data are known as normed vector spaces and inner product spaces, respectively.

Coordinate space Fn can be equipped with the standard dot product:

⟨𝐱,𝐲⟩=𝐱⋅𝐲=x1y1+⋯+xnyn.

In 𝐑2, this reflects the common notion of the angle between two vectors 𝐱 and 𝐲, by the law of cosines:

𝐱⋅𝐲=cos⁡(∠(𝐱,𝐲))⋅|𝐱|⋅|𝐲|.

Because of this, two vectors satisfying ⟨𝐱,𝐲⟩=0 are called orthogonal. An important variant of the standard dot product is used in Minkowski space: 𝐑4 endowed with the Lorentz product

⟨𝐱|𝐲⟩=x1y1+x2y2+x3y3−x4y4.

In contrast to the standard dot product, it is not positive definite: ⟨𝐱|𝐱⟩ also takes negative values, for example, for 𝐱=(0,0,0,1). Singling out the fourth coordinate—corresponding to time, as opposed to three space-dimensions—makes it useful for the mathematical treatment of special relativity. Note that in other conventions time is often written as the first, or "zeroeth" component so that the Lorentz product is written ⟨𝐱|𝐲⟩=−x0y0+x1y1+x2y2+x3y3.

### Topological vector spaces

Convergence questions are treated by considering vector spaces V carrying a compatible topology, a structure that allows one to talk about elements being close to each other. Compatible here means that addition and scalar multiplication have to be continuous maps. Roughly, if 𝐱 and 𝐲 in V, and a in F vary by a bounded amount, then so do 𝐱+𝐲 and a𝐱. To make sense of specifying the amount a scalar changes, the field F also has to carry a topology in this context; a common choice is the reals or the complex numbers.

In such topological vector spaces one can consider series of vectors. The infinite sum

∑i=1∞fi = limn→∞f1+⋯+fn

denotes the limit of the corresponding finite partial sums of the sequence f1,f2,… of elements of V. For example, the fi could be (real or complex) functions belonging to some function space V, in which case the series is a function series. The mode of convergence of the series depends on the topology imposed on the function space. In such cases, pointwise convergence and uniform convergence are two prominent examples.

A way to ensure the existence of limits of certain infinite series is to restrict attention to spaces where any Cauchy sequence has a limit; such a vector space is called complete. Roughly, a vector space is complete provided that it contains all necessary limits. For example, the vector space of polynomials on the unit interval [0,1], equipped with the topology of uniform convergence is not complete because any continuous function on [0,1] can be uniformly approximated by a sequence of polynomials, by the Weierstrass approximation theorem.  In contrast, the space of all continuous functions on [0,1] with the same topology is complete. A norm gives rise to a topology by defining that a sequence of vectors 𝐯n converges to 𝐯 if and only if

limn→∞|𝐯n−𝐯|=0.

Banach and Hilbert spaces are complete topological vector spaces whose topologies are given, respectively, by a norm and an inner product. Their study—a key piece of functional analysis—focuses on infinite-dimensional vector spaces, since all norms on finite-dimensional topological vector spaces give rise to the same notion of convergence. The image at the right shows the equivalence of the 1-norm and ∞-norm on 𝐑2: as the unit "balls" enclose each other, a sequence converges to zero in one norm if and only if it so does in the other norm. In the infinite-dimensional case, however, there will generally be inequivalent topologies, which makes the study of topological vector spaces richer than that of vector spaces without additional data.

From a conceptual point of view, all notions related to topological vector spaces should match the topology. For example, instead of considering all linear maps (also called functionals) V→W, maps between topological vector spaces are required to be continuous. In particular, the (topological) dual space V∗ consists of continuous functionals V→𝐑 (or to 𝐂). The fundamental Hahn–Banach theorem is concerned with separating subspaces of appropriate topological vector spaces by continuous functionals.

#### Banach spaces

Banach spaces, introduced by Stefan Banach, are complete normed vector spaces.

A first example is the vector space ℓp consisting of infinite vectors with real entries 𝐱=(x1,x2,…,xn,…)

whose p-norm (1≤p≤∞) given by

‖𝐱‖∞:=supi|xi| for p=∞, and

‖𝐱‖p:=(∑i|xi|p)1p for p<∞.

The topologies on the infinite-dimensional space ℓp are inequivalent for different p. For example, the sequence of vectors 𝐱n=(2−n,2−n,…,2−n,0,0,…), in which the first 2n components are 2−n and the following ones are 0, converges to the zero vector for p=∞, but does not for p=1:

‖𝐱n‖∞=sup⁡(2−n,0)=2−n→0,

but

‖𝐱n‖1=∑i=12n2−n=2n⋅2−n=1.

More generally than sequences of real numbers, functions f:Ω→ℝ are endowed with a norm that replaces the above sum by the Lebesgue integral

‖f‖p:=(∫Ω|f(x)|pdμ(x))1p.

The space of integrable functions on a given domain Ω (for example an interval) satisfying ‖f‖p<∞, and equipped with this norm are called Lebesgue spaces, denoted Lp(Ω).

These spaces are complete. (If one uses the Riemann integral instead, the space is not complete, which may be seen as a justification for Lebesgue's integration theory.) Concretely this means that for any sequence of Lebesgue-integrable functions f1,f2,…,fn,… with ‖fn‖p<∞, satisfying the condition

limk, n→∞∫Ω|fk(x)−fn(x)|pdμ(x)=0

there exists a function f(x) belonging to the vector space Lp(Ω) such that

limk→∞∫Ω|f(x)−fk(x)|pdμ(x)=0.

Imposing boundedness conditions not only on the function, but also on its derivatives leads to Sobolev spaces.

#### Hilbert spaces

Complete inner product spaces are known as Hilbert spaces, in honor of David Hilbert. The Hilbert space L2(Ω), with inner product given by

⟨f , g⟩=∫Ωf(x)g(x)‾dx,

where g(x)‾ denotes the complex conjugate of g(x), is a key case.

By definition, in a Hilbert space, any Cauchy sequence converges to a limit. Conversely, finding a sequence of functions fn with desirable properties that approximate a given limit function is equally crucial. Early analysis, in the guise of the Taylor approximation, established an approximation of differentiable functions f by polynomials. By the Stone–Weierstrass theorem, every continuous function on [a,b] can be approximated as closely as desired by a polynomial. A similar approximation technique by trigonometric functions is commonly called Fourier expansion, and is much applied in engineering. More generally, and more conceptually, the theorem yields a simple description of what "basic functions", or, in abstract Hilbert spaces, what basic vectors suffice to generate a Hilbert space H, in the sense that the closure of their span (that is, finite linear combinations and limits of those) is the whole space. Such a set of functions is called a basis of H, its cardinality is known as the Hilbert space dimension. Not only does the theorem exhibit suitable basis functions as sufficient for approximation purposes, but also together with the Gram–Schmidt process, it enables one to construct a basis of orthogonal vectors. Such orthogonal bases are the Hilbert space generalization of the coordinate axes in finite-dimensional Euclidean space.

The solutions to various differential equations can be interpreted in terms of Hilbert spaces. For example, a great many fields in physics and engineering lead to such equations, and frequently solutions with particular physical properties are used as basis functions, often orthogonal. As an example from physics, the time-dependent Schrödinger equation in quantum mechanics describes the change of physical properties in time by means of a partial differential equation, whose solutions are called wavefunctions. Definite values for physical properties such as energy, or momentum, correspond to eigenvalues of a certain (linear) differential operator and the associated wavefunctions are called eigenstates. The spectral theorem decomposes a linear compact operator acting on functions in terms of these eigenfunctions and their eigenvalues.

### Algebras over fields

General vector spaces do not possess a multiplication between vectors. A vector space equipped with an additional bilinear operator defining the multiplication of two vectors is an algebra over a field (or F-algebra if the field F is specified).

For example, the set of all polynomials p(t) forms an algebra known as the polynomial ring: using that the sum of two polynomials is a polynomial, they form a vector space; they form an algebra since the product of two polynomials is again a polynomial. Rings of polynomials (in several variables) and their quotients form the basis of algebraic geometry, because they are rings of functions of algebraic geometric objects.

Another crucial example are Lie algebras, which are neither commutative nor associative, but the failure to be so is limited by the constraints ([x,y] denotes the product of x and y):

[x,y]=−[y,x] (anticommutativity), and

[x,[y,z]]+[y,[z,x]]+[z,[x,y]]=0 (Jacobi identity).

Examples include the vector space of n-by-n matrices, with [x,y]=xy−yx, the commutator of two matrices, and 𝐑3, endowed with the cross product.

The tensor algebra T⁡(V) is a formal way of adding products to any vector space V to obtain an algebra. As a vector space, it is spanned by symbols, called simple tensors

𝐯1⊗𝐯2⊗⋯⊗𝐯n,

where the degree n varies.

The multiplication is given by concatenating such symbols, imposing the distributive law under addition, and requiring that scalar multiplication commute with the tensor product ⊗, much the same way as with the tensor product of two vector spaces introduced in the above section on tensor products. In general, there are no relations between 𝐯1⊗𝐯2 and 𝐯2⊗𝐯1. Forcing two such elements to be equal leads to the symmetric algebra, whereas forcing 𝐯1⊗𝐯2=−𝐯2⊗𝐯1 yields the exterior algebra.

### Vector bundles

A vector bundle is a family of vector spaces parametrized continuously by a topological space X. More precisely, a vector bundle over X is a topological space E equipped with a continuous map

π:E→X

such that for every x in X, the fiber π−1(x) is a vector space. The case dim V = 1 is called a line bundle. For any vector space V, the projection X × V → X makes the product X × V into a "trivial" vector bundle. Vector bundles over X are required to be locally a product of X and some (fixed) vector space V: for every x in X, there is a neighborhood U of x such that the restriction of π to π−1(U) is isomorphic to the trivial bundle U × V → U. Despite their locally trivial character, vector bundles may (depending on the shape of the underlying space X) be "twisted" in the large (that is, the bundle need not be (globally isomorphic to) the trivial bundle X × V). For example, the Möbius strip can be seen as a line bundle over the circle S1 (by identifying open intervals with the real line). It is, however, different from the cylinder S1 × R, because the latter is orientable whereas the former is not.

Properties of certain vector bundles provide information about the underlying topological space. For example, the tangent bundle consists of the collection of tangent spaces parametrized by the points of a differentiable manifold. The tangent bundle of the circle S1 is globally isomorphic to S1 × R, since there is a global nonzero vector field on S1. In contrast, by the hairy ball theorem, there is no (tangent) vector field on the 2-sphere S2 which is everywhere nonzero. K-theory studies the isomorphism classes of all vector bundles over some topological space. In addition to deepening topological and geometrical insight, it has purely algebraic consequences, such as the classification of finite-dimensional real division algebras: R, C, the quaternions H and the octonions O.

The cotangent bundle of a differentiable manifold consists, at every point of the manifold, of the dual of the tangent space, the cotangent space. Sections of that bundle are known as differential one-forms.

### Modules

Modules are to rings what vector spaces are to fields: the same axioms, applied to a ring R instead of a field F, yield modules. The theory of modules, compared to that of vector spaces, is complicated by the presence of ring elements that do not have multiplicative inverses. For example, modules need not have bases, as the Z-module (that is, abelian group) Z/2Z shows; those modules that do (including all vector spaces) are known as free modules. Nevertheless, a vector space can be compactly defined as a module over a ring which is a field, with the elements being called vectors.  Some authors use the term vector space to mean modules over a division ring. The algebro-geometric interpretation of commutative rings via their spectrum allows the development of concepts such as locally free modules, the algebraic counterpart to vector bundles.

### Affine and projective spaces

Roughly, affine spaces are vector spaces whose origins are not specified. More precisely, an affine space is a set with a free transitive vector space action. In particular, a vector space is an affine space over itself, by the map

V×V→W,(𝐯,𝐚)↦𝐚+𝐯.

If W is a vector space, then an affine subspace is a subset of W obtained by translating a linear subspace V by a fixed vector x ∈ W; this space is denoted by x + V (it is a coset of V in W) and consists of all vectors of the form x + v for v ∈ V. An important example is the space of solutions of a system of inhomogeneous linear equations

A𝐯=𝐛

generalizing the homogeneous case discussed in the above section on linear equations, which can be found by setting 𝐛=𝟎 in this equation. The space of solutions is the affine subspace x + V where x is a particular solution of the equation, and V is the space of solutions of the homogeneous equation (the nullspace of A).

The set of one-dimensional subspaces of a fixed finite-dimensional vector space V is known as projective space; it may be used to formalize the idea of parallel lines intersecting at infinity. Grassmannians and flag manifolds generalize this by parametrizing linear subspaces of fixed dimension k and flags of subspaces, respectively.
