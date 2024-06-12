# Iterator
* allows to perform tasks on a sequence of items in turn, i.e. iterates
* is responsible for the logic of iteration and determining when the sequence has finished
* iterators are lazy - they have no effect until methods are called on that iterator
* all iterators implement trait __Iterator__, which has _next()_ method that gets called to traverse data
* some methods consume the iterator, while others produce a new iterator from the provided iterator
    * _into_iter_ consumes the collection. Once the collection is consumed, it is no longer available for reuse, because owenrshi has been moved into the loop that has ceased to exist
    * _iter_ borrows each element of the collection, leaving the collection untouched and available for reuse after the loop
    * _iter_mut_ mutably borrows each element of the collection, and could modify the collection while leaving it available for later reuse (modify the collection in place)
